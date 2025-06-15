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
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "webp")]
    WebP,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    #[serde(rename = "type")]
    pub format: Vec<ImageFormat>,
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
