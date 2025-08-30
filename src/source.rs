use crate::client::Client;
use crate::error::Result;
use crate::options::{ConvertOptions, PreserveOptions, ResizeOptions, StoreOptions};
use crate::result::TinifyResult;
use std::sync::Arc;
use tracing::{info, instrument};

/// Represents an image source uploaded to Tinify
///
/// `Source` objects represent images that have been uploaded to Tinify servers,
/// allowing various operations such as resizing, format conversion, metadata preservation, etc.
#[derive(Debug, Clone)]
pub struct Source {
    location: String,
    client: Arc<Client>,
}

impl Source {
    /// Create a new Source object
    ///
    /// # Arguments
    ///
    /// * `location` - Image location URL on Tinify servers
    /// * `client` - Arc reference to the HTTP client
    pub fn new(location: String, client: Arc<Client>) -> Self {
        Self { location, client }
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.png").await?;
    ///
    /// let resize_options = ResizeOptions {
    ///     method: ResizeMethod::Fit,
    ///     width: Some(300),
    ///     height: Some(200),
    /// };
    ///
    /// let result = source.resize(resize_options).await?;
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location))]
    pub async fn resize(&self, options: ResizeOptions) -> Result<TinifyResult> {
        info!("Resizing image at location: {}", self.location);

        // Validate resize options
        crate::Tinify::validate_dimensions(options.width, options.height)?;

        let body = serde_json::to_vec(&serde_json::json!({ "resize": options }))?;
        let response = self.client.post(&self.location, Some(body)).await?;
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.png").await?;
    ///
    /// let convert_options = ConvertOptions {
    ///     format: ImageFormat::Jpeg,
    ///     background: Some("#FFFFFF".to_string()),
    /// };
    ///
    /// let result = source.convert(convert_options).await?;
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location))]
    pub async fn convert(&self, options: ConvertOptions) -> Result<TinifyResult> {
        info!("Converting image format at location: {}", self.location);

        let body = serde_json::to_vec(&serde_json::json!({ "convert": options }))?;
        let response = self.client.post(&self.location, Some(body)).await?;
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.jpg").await?;
    ///
    /// let preserve_options = PreserveOptions {
    ///     preserve: vec![
    ///         PreserveMetadata::Copyright,
    ///         PreserveMetadata::Location,
    ///     ],
    /// };
    ///
    /// let result = source.preserve(preserve_options).await?;
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location))]
    pub async fn preserve(&self, options: PreserveOptions) -> Result<TinifyResult> {
        info!(
            "Preserving metadata for image at location: {}",
            self.location
        );

        let body = serde_json::to_vec(&options)?;
        let response = self.client.post(&self.location, Some(body)).await?;
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.jpg").await?;
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
    /// let result = source.store(StoreOptions::S3(s3_options)).await?;
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location))]
    pub async fn store(&self, options: StoreOptions) -> Result<TinifyResult> {
        info!(
            "Storing image to cloud storage from location: {}",
            self.location
        );

        let body = serde_json::to_vec(&options)?;
        let response = self.client.post(&self.location, Some(body)).await?;
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.png").await?;
    ///
    /// let image_data = source.to_buffer().await?;
    /// println!("Image size: {} bytes", image_data.len());
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location))]
    pub async fn to_buffer(&self) -> Result<Vec<u8>> {
        info!("Downloading image data from location: {}", self.location);

        let response = self.client.get(&self.location).await?;
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
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.png").await?;
    ///
    /// source.to_file("output.png").await?;
    /// println!("Image saved to output.png");
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    #[instrument(skip(self), fields(location = %self.location, path = %path.as_ref().display()))]
    pub async fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let path_display = path.as_ref().display().to_string();
        info!(
            "Saving image from location {} to file: {}",
            self.location, path_display
        );

        let response = self.client.get(&self.location).await?;
        let mut result = TinifyResult::new(response);
        result.to_file(path).await
    }

    /// Get the location URL for this source
    pub fn location(&self) -> &str {
        &self.location
    }
}
