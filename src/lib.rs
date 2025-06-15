mod client;
mod error;
mod options;
mod result;
mod source;

pub use error::{Result, TinifyError};
pub use options::{
    ConvertOptions, GCSOptions, ImageFormat, PreserveMetadata, PreserveOptions, ResizeMethod,
    ResizeOptions, S3Options, StoreOptions,
};
pub use result::TinifyResult;
pub use source::Source;

use client::{get_client, set_client, Client};
use serde_json::json;
use std::path::Path;

pub struct Tinify;

impl Tinify {
    pub async fn set_key(api_key: String) -> Result<()> {
        let client = Client::new(api_key)?;
        set_client(client).await
    }

    pub async fn set_app_identifier(app_identifier: String) -> Result<()> {
        let mut client = Client::new(get_client().await?.api_key().to_string())?;
        client.set_app_identifier(app_identifier);
        set_client(client).await
    }

    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Source> {
        let data = tokio::fs::read(path).await?;
        Self::from_buffer(data).await
    }

    pub async fn from_buffer(data: Vec<u8>) -> Result<Source> {
        let client = get_client().await?;
        let response = client
            .post("https://api.tinify.com/shrink", Some(data))
            .await?;

        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| TinifyError::UnknownError {
                message: "No Location header in response".to_string(),
            })?;

        Ok(Source::new(location.to_string()))
    }

    pub async fn from_url(url: &str) -> Result<Source> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&json!({ "source": { "url": url } }))?;
        let response = client
            .post("https://api.tinify.com/shrink", Some(body))
            .await?;

        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| TinifyError::UnknownError {
                message: "No Location header in response".to_string(),
            })?;

        Ok(Source::new(location.to_string()))
    }
}
