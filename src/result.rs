use anyhow::Result;
use reqwest::Response;
use std::path::Path;

pub struct TinifyResult {
    response: Response,
}

impl TinifyResult {
    pub fn new(response: Response) -> Self {
        Self { response }
    }

    pub async fn to_buffer(&mut self) -> Result<Vec<u8>> {
        // 由于 reqwest::Response 只能被消费一次，这里通过 take() 移出 response
        let response = std::mem::replace(&mut self.response, unsafe {
            // 创建一个空的 Response 占位，后续方法不应再使用 self.response
            std::mem::zeroed()
        });
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn to_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let bytes = self.to_buffer().await?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }

    pub fn compression_count(&self) -> Option<u32> {
        self.response
            .headers()
            .get("Compression-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    pub fn image_width(&self) -> Option<u32> {
        self.response
            .headers()
            .get("Image-Width")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    pub fn image_height(&self) -> Option<u32> {
        self.response
            .headers()
            .get("Image-Height")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    pub fn content_type(&self) -> Option<String> {
        self.response
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
    }

    pub fn content_length(&self) -> Option<u64> {
        self.response
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }
}
