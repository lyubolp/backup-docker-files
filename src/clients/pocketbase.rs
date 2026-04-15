use reqwest::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;

pub struct PocketbaseClient {
    base_url: String,
    username: String,
    password: String,
    reqwest_client: Client,
}

impl PocketbaseClient {
    pub fn new(base_url: &str, username: &str, password: &str) -> Self {
        PocketbaseClient {
            base_url: base_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            reqwest_client: Client::new(),
        }
    }

    pub async fn send_get_request_to_pocketbase(
        &self,
        endpoint: &str,
        token: Option<&str>,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        let response = self
            .reqwest_client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        Ok(response)
    }

    pub async fn send_post_request_to_pocketbase(
        &self,
        endpoint: &str,
        body: &HashMap<&str, String>,
        token: Option<&str>,
    ) -> Result<Response, String> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                "authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        let response = self
            .reqwest_client
            .post(url)
            .json(&body)
            .headers(headers)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        Ok(response)
    }
    pub async fn auth_with_password(&self) -> Result<String, String> {
        let endpoint = "collections/_superusers/auth-with-password";

        let body: HashMap<&str, String> = HashMap::from([
            ("identity", self.username.clone()),
            ("password", self.password.clone()),
        ]);

        let response = self
            .send_post_request_to_pocketbase(endpoint, &body, None)
            .await?;

        if response.status().is_success() {
            let parsed: Value = serde_json::from_str(&response.text().await.unwrap())
                .map_err(|e| format!("Failed to parse response: {e}"))?;

            match parsed.get("token") {
                Some(token) => Ok(token.as_str().unwrap_or_default().to_string()),
                None => Err(format!("Token not found in response: {:?}", parsed)),
            }
        } else {
            Err(format!(
                "Error when calling auth with password: {:?}",
                response
            ))
        }
    }
}
