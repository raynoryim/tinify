use crate::client::get_client;
use crate::options::{ConvertOptions, PreserveOptions, ResizeOptions, StoreOptions};
use crate::result::TinifyResult;
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;

pub struct Source {
    location: String,
}

impl Source {
    pub fn new(location: String) -> Self {
        Self { location }
    }

    pub async fn resize(&self, options: ResizeOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    pub async fn convert(&self, options: ConvertOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    pub async fn preserve(&self, options: PreserveOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    pub async fn store(&self, options: StoreOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    pub async fn to_buffer(&self) -> Result<Vec<u8>> {
        let client = get_client().await?;
        let response = client.get(&self.location).await?;
        let mut result = TinifyResult::new(response);
        result.to_buffer().await
    }

    pub async fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let client = get_client().await?;
        let response = client.get(&self.location).await?;
        let mut result = TinifyResult::new(response);
        result.to_file(path).await
    }
}
