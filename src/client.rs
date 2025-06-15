use crate::error::{Result, TinifyError};
use reqwest::{Client as ReqwestClient, Response};
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct Client {
    http_client: ReqwestClient,
    api_key: String,
    app_identifier: Option<String>,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self> {
        let http_client = ReqwestClient::builder()
            .build()
            .map_err(TinifyError::ConnectionError)?;

        Ok(Self {
            http_client,
            api_key,
            app_identifier: None,
        })
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn set_app_identifier(&mut self, app_identifier: String) {
        self.app_identifier = Some(app_identifier);
    }

    pub async fn post(&self, url: &str, body: Option<Vec<u8>>) -> Result<Response> {
        let mut request = self.http_client.post(url);

        // Add authorization header
        let auth = format!("api:{}", self.api_key);
        let auth_header = format!("Basic {}", base64::encode(auth));
        request = request.header("Authorization", auth_header);

        // Add app identifier if set
        if let Some(ref app_id) = self.app_identifier {
            request = request.header("User-Agent", app_id);
        }

        // Add body if provided
        if let Some(body_data) = body {
            request = request.body(body_data);
        }

        let response = request.send().await.map_err(TinifyError::ConnectionError)?;

        // Check for API errors
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body = response
                .json::<serde_json::Value>()
                .await
                .unwrap_or_default();

            let message = error_body
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            let error_type = error_body
                .get("error")
                .and_then(|e| e.as_str())
                .map(String::from);

            return match status {
                401 | 429 => Err(TinifyError::AccountError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                400..=499 => Err(TinifyError::ClientError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                500..=599 => Err(TinifyError::ServerError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                _ => Err(TinifyError::UnknownError { message }),
            };
        }

        Ok(response)
    }

    pub async fn get(&self, url: &str) -> Result<Response> {
        let mut request = self.http_client.get(url);

        // Add authorization header
        let auth = format!("api:{}", self.api_key);
        let auth_header = format!("Basic {}", base64::encode(auth));
        request = request.header("Authorization", auth_header);

        // Add app identifier if set
        if let Some(ref app_id) = self.app_identifier {
            request = request.header("User-Agent", app_id);
        }

        let response = request.send().await.map_err(TinifyError::ConnectionError)?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body = response
                .json::<serde_json::Value>()
                .await
                .unwrap_or_default();

            let message = error_body
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            let error_type = error_body
                .get("error")
                .and_then(|e| e.as_str())
                .map(String::from);

            return match status {
                401 | 429 => Err(TinifyError::AccountError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                400..=499 => Err(TinifyError::ClientError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                500..=599 => Err(TinifyError::ServerError {
                    message,
                    error_type,
                    status: Some(status),
                }),
                _ => Err(TinifyError::UnknownError { message }),
            };
        }

        Ok(response)
    }
}

// Global client instance
static CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn get_client() -> Result<Arc<Client>> {
    CLIENT
        .get_or_try_init(|| async {
            Err(TinifyError::UnknownError {
                message: "Client not initialized. Call Tinify::set_key first.".to_string(),
            })
        })
        .await
        .map(Arc::clone)
}

pub async fn set_client(client: Client) -> Result<()> {
    CLIENT
        .set(Arc::new(client))
        .map_err(|_| TinifyError::UnknownError {
            message: "Client already initialized".to_string(),
        })
}
