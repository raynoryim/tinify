use crate::client::get_client;
use crate::error::Result;
use crate::options::{ConvertOptions, PreserveOptions, ResizeOptions, StoreOptions};
use crate::result::TinifyResult;

/// Represents an image source uploaded to Tinify
///
/// `Source` objects represent images that have been uploaded to Tinify servers,
/// allowing various operations such as resizing, format conversion, metadata preservation, etc.
#[derive(Debug)]
pub struct Source {
    location: String,
}

impl Source {
    /// Create a new Source object
    ///
    /// # Arguments
    ///
    /// * `location` - Image location URL on Tinify servers
    pub fn new(location: String) -> Self {
        Self { location }
    }

    /// Resize the image
    ///
    /// Resize the image according to the provided options, supporting multiple resizing methods.
    ///
    /// # Arguments
    ///
    /// * `options` - Resize options including resize method, width, height, etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::{Tinify, ResizeOptions, ResizeMethod};
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.png").await.unwrap();
    ///
    /// let resize_options = ResizeOptions {
    ///     method: ResizeMethod::Fit,
    ///     width: Some(300),
    ///     height: Some(200),
    /// };
    ///
    /// let result = source.resize(resize_options).await.unwrap();
    /// # });
    /// ```
    pub async fn resize(&self, options: ResizeOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&serde_json::json!({ "resize": options }))?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    /// Convert image format
    ///
    /// Convert the image to the specified format (such as JPEG, PNG, WebP, AVIF, etc.).
    ///
    /// # Arguments
    ///
    /// * `options` - Format conversion options including target format and background color
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::{Tinify, ConvertOptions, ImageFormat};
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.png").await.unwrap();
    ///
    /// let convert_options = ConvertOptions {
    ///     format: ImageFormat::Jpeg,
    ///     background: Some("#FFFFFF".to_string()),
    /// };
    ///
    /// let result = source.convert(convert_options).await.unwrap();
    /// # });
    /// ```
    pub async fn convert(&self, options: ConvertOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&serde_json::json!({ "convert": options }))?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    /// Preserve image metadata
    ///
    /// Preserve specified image metadata information during compression.
    ///
    /// # Arguments
    ///
    /// * `options` - Metadata preservation options specifying which metadata types to preserve
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::{Tinify, PreserveOptions, PreserveMetadata};
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.jpg").await.unwrap();
    ///
    /// let preserve_options = PreserveOptions {
    ///     preserve: vec![PreserveMetadata::Copyright, PreserveMetadata::Creation],
    /// };
    ///
    /// let result = source.preserve(preserve_options).await.unwrap();
    /// # });
    /// ```
    pub async fn preserve(&self, options: PreserveOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    /// Store image to cloud storage service
    ///
    /// Store the processed image directly to cloud storage services like Amazon S3 or Google Cloud Storage.
    ///
    /// # Arguments
    ///
    /// * `options` - Storage options including cloud service provider and related configurations
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::{Tinify, StoreOptions, S3Options};
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.jpg").await.unwrap();
    ///
    /// let s3_options = S3Options {
    ///     aws_access_key_id: "your-access-key".to_string(),
    ///     aws_secret_access_key: "your-secret-key".to_string(),
    ///     region: "us-east-1".to_string(),
    ///     path: "bucket/path/image.jpg".to_string(),
    ///     headers: None,
    ///     acl: Some("public-read".to_string()),
    /// };
    ///
    /// let result = source.store(StoreOptions::S3(s3_options)).await.unwrap();
    /// # });
    /// ```
    pub async fn store(&self, options: StoreOptions) -> Result<TinifyResult> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&options)?;
        let response = client.post(&self.location, Some(body)).await?;
        Ok(TinifyResult::new(response))
    }

    /// Get image data to memory buffer
    ///
    /// Download processed image data to a byte array in memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.png").await.unwrap();
    ///
    /// let image_data = source.to_buffer().await.unwrap();
    /// println!("Image size: {} bytes", image_data.len());
    /// # });
    /// ```
    pub async fn to_buffer(&self) -> Result<Vec<u8>> {
        let client = get_client().await?;
        let response = client.get(&self.location).await?;
        let mut result = TinifyResult::new(response);
        result.to_buffer().await
    }

    /// Save image to local file
    ///
    /// Download processed image and save it to the specified local file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Local file path to save the image
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.png").await.unwrap();
    ///
    /// source.to_file("output.png").await.unwrap();
    /// println!("Image saved to output.png");
    /// # });
    /// ```
    pub async fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let client = get_client().await?;
        let response = client.get(&self.location).await?;
        let mut result = TinifyResult::new(response);
        result.to_file(path).await
    }
}
