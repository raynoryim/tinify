use crate::error::{Result, TinifyError};
use base64::Engine;
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

    /// Generic method for handling HTTP response errors
    async fn handle_error_response(response: Response) -> Result<Response> {
        if response.status().is_success() {
            return Ok(response);
        }

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

        match status {
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
        }
    }

    /// Helper method to add common request headers
    fn add_common_headers(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        // Add authorization header
        let auth = format!("api:{}", self.api_key);
        let auth_header = format!(
            "Basic {}",
            Engine::encode(&base64::engine::general_purpose::STANDARD, auth)
        );
        let request = request.header("Authorization", auth_header);

        // Add application identifier header (if set)
        match &self.app_identifier {
            Some(app_id) => request.header("User-Agent", app_id),
            None => request,
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn set_app_identifier(&mut self, app_identifier: String) {
        self.app_identifier = Some(app_identifier);
    }

    pub async fn post<S: AsRef<str>>(&self, url: S, body: Option<Vec<u8>>) -> Result<Response> {
        let request = self.http_client.post(url.as_ref());
        let mut request = self.add_common_headers(request);

        // Add request body (if provided)
        if let Some(body_data) = body {
            // Check if request body is JSON format (starts with { or [)
            if body_data.starts_with(b"{") || body_data.starts_with(b"[") {
                request = request.header("Content-Type", "application/json");
            }
            request = request.body(body_data);
        }

        let response = request.send().await.map_err(TinifyError::ConnectionError)?;
        Self::handle_error_response(response).await
    }

    pub async fn get<S: AsRef<str>>(&self, url: S) -> Result<Response> {
        let request = self.http_client.get(url.as_ref());
        let request = self.add_common_headers(request);

        let response = request.send().await.map_err(TinifyError::ConnectionError)?;
        Self::handle_error_response(response).await
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
