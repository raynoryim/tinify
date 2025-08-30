use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResizeMethod {
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "thumb")]
    Thumb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeOptions {
    pub method: ResizeMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            method: ResizeMethod::Fit,
            width: Some(100),
            height: Some(100),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    #[serde(rename = "image/avif")]
    Avif,
    #[serde(rename = "image/webp")]
    WebP,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    #[serde(rename = "type")]
    pub format: ImageFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreserveMetadata {
    #[serde(rename = "copyright")]
    Copyright,
    #[serde(rename = "creation")]
    Creation,
    #[serde(rename = "location")]
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreserveOptions {
    pub preserve: Vec<PreserveMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Options {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub region: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCSOptions {
    pub gcp_access_token: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "service")]
pub enum StoreOptions {
    #[serde(rename = "s3")]
    S3(S3Options),
    #[serde(rename = "gcs")]
    GCS(GCSOptions),
}
