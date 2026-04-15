/*
1. Get superuser token - POST /api/collections/_superusers/auth-with-password
2. Create backup - POST /api/backups
3. Get backups - GET /api/backups
4. Parse key from backups
5. Get file access token - POST /api/files/token
6. Download backup file - GET http://0.0.0.0:8080/api/backups/<backup_key>?token=<file_access_token>
*/

use super::Extractor;
use crate::clients::pocketbase::PocketbaseClient;
use crate::constants;
use crate::models::container::Container;
use crate::services::discovery::get_docker_client;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub(crate) struct PocketbaseExtractor;

impl Extractor for PocketbaseExtractor {
    async fn extract(container: &Container) -> Result<String, String> {
        let pocketbase_url = match &container.labels.other {
            Some(other_labels) => other_labels.get("pocketbase.url"),
            None => None,
        }
        .ok_or("Pocketbase URL not found in container labels".to_string())?;

        let pocketbase_username = match &container.labels.other {
            Some(other_labels) => other_labels.get("pocketbase.username"),
            None => None,
        }
        .ok_or("Pocketbase username not found in container labels".to_string())?;

        // TODO: This is not okay, we should use secrets instead of labels for this
        let pocketbase_password = match &container.labels.other {
            Some(other_labels) => other_labels.get("pocketbase.password"),
            None => None,
        }
        .ok_or("Pocketbase password not found in container labels".to_string())?;

        let docker = get_docker_client()?;

        if container.labels.online {
            let pause_result = docker.pause_container(&container.name).await;

            if pause_result.is_err() {
                return Err(format!(
                    "Failed to pause container {}: {}",
                    container.name,
                    pause_result.err().unwrap()
                ));
            }
        }

        let staging_path = format!("{}/{}", constants::STAGING_DIR, container.name);

        let pb_client =
            PocketbaseClient::new(pocketbase_url, pocketbase_username, pocketbase_password);

        let token = get_superuser_token(&pb_client).await?;
        let backup_key = create_backup(&pb_client, &token).await?;

        let file_access_token = get_file_access_token(&pb_client, &token).await?;
        let result_path = download_backup_file(
            &pb_client,
            &token,
            &backup_key,
            &file_access_token,
            &staging_path,
        )
        .await?;

        if container.labels.online {
            let unpause_result = docker.unpause_container(&container.name).await;

            if unpause_result.is_err() {
                return Err(format!(
                    "Failed to unpause container {}: {}",
                    container.name,
                    unpause_result.err().unwrap()
                ));
            }
        }

        Ok(result_path)
    }
}

async fn get_superuser_token(pb_client: &PocketbaseClient) -> Result<String, String> {
    pb_client.auth_with_password().await
}

async fn create_backup(pb_client: &PocketbaseClient, token: &str) -> Result<String, String> {
    let backup_name = format!("bdf-backup-{}.zip", Uuid::new_v4());
    let body = HashMap::from([("name", backup_name.clone())]);
    let response = pb_client
        .send_post_request_to_pocketbase("backups", &body, Some(token))
        .await?;

    if response.status().is_success() {
        Ok(backup_name)
    } else {
        Err(format!(
            "Failed to create backup: {}",
            response.text().await.unwrap_or_default()
        ))
    }
}

async fn get_file_access_token(
    pb_client: &PocketbaseClient,
    token: &str,
) -> Result<String, String> {
    let response = pb_client
        .send_post_request_to_pocketbase("files/token", &HashMap::new(), Some(token))
        .await?;

    if response.status().is_success() {
        let parsed: serde_json::Value = serde_json::from_str(&response.text().await.unwrap())
            .map_err(|e| format!("Failed to parse response: {e}"))?;
        match parsed.get("token") {
            Some(token) => Ok(token.as_str().unwrap_or_default().to_string()),
            None => Err(format!("Token not found in response: {:?}", parsed)),
        }
    } else {
        Err(format!(
            "Failed to get file access token: {}",
            response.text().await.unwrap_or_default()
        ))
    }
}

async fn download_backup_file(
    pb_client: &PocketbaseClient,
    token: &str,
    backup_file: &str,
    file_token: &str,
    staging_path: &str,
) -> Result<String, String> {
    let response = pb_client
        .send_get_request_to_pocketbase(
            &format!("backups/{}?token={}", backup_file, file_token),
            Some(token),
        )
        .await?;

    if response.status().is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read backup response bytes: {e}"))?;

        std::fs::create_dir_all(staging_path)
            .map_err(|e| format!("Failed to create staging directory {staging_path}: {e}"))?;

        let dest = Path::new(staging_path).join(backup_file);
        std::fs::write(&dest, &bytes)
            .map_err(|e| format!("Failed to write backup file {}: {e}", dest.display()))?;

        Ok(staging_path.to_string())
    } else {
        Err(format!(
            "Failed to download backup file: {}",
            response.text().await.unwrap_or_default()
        ))
    }
}
