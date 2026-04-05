use std::vec;

use super::utils::create_staging_dir;
use crate::constants;
use crate::models::container::Container;
use crate::services::discovery::get_docker_client;
use crate::services::extraction::file::copy_file;

pub async fn extract(container: &Container) -> Result<String, String> {
    if let Ok(docker) = get_docker_client() {
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

        let staging_result = create_staging_dir();

        if staging_result.is_err() {
            return Err(format!(
                "Failed to create staging directory: {}",
                staging_result.err().unwrap()
            ));
        }

        let staging_path = format!("{}/{}", constants::STAGING_DIR, container.name);

        let mut errors: Vec<String> = vec![];
        for path in container.labels.file_paths.iter() {
            if let Err(e) = copy_file(container, path, &staging_path, &docker).await {
                errors.push(format!("Failed to copy file {}: {}", path, e));
            }
        }
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

        if errors.len() == 0 {
            Ok(staging_path)
        } else {
            Err(errors.join("; "))
        }
    } else {
        Err("Failed to connect to Docker".to_string())
    }
}
