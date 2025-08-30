use crate::error::Result;
use reqwest::Response;
use std::path::Path;

/// Represents the result of Tinify API operations
///
/// `TinifyResult` contains response data and metadata after API operations,
/// and can be used to retrieve processed image data, metadata information, etc.
#[derive(Debug)]
pub struct TinifyResult {
    response: Option<Response>,
}

impl TinifyResult {
    /// Create a new TinifyResult object
    ///
    /// # Arguments
    ///
    /// * `response` - HTTP response object
    pub fn new(response: Response) -> Self {
        Self {
            response: Some(response),
        }
    }

    /// Get image data to memory buffer
    ///
    /// Read the image data from the response into a byte array.
    /// Note: This method consumes the response data and can only be called once.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// let client = Tinify::new("your-api-key".to_string())?;
    /// let source = client.source_from_file("input.png").await?;
    /// let mut result = source.resize(Default::default()).await?;
    ///
    /// let image_data = result.to_buffer().await?;
    /// println!("Image size: {} bytes", image_data.len());
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    pub async fn to_buffer(&mut self) -> Result<Vec<u8>> {
        // Since reqwest::Response can only be consumed once, we use take() to move out the response
        let response = self.response.take().expect("Response has been consumed");
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Save image to local file
    ///
    /// Save the image data from the response to the specified local file path.
    /// Note: This method consumes the response data and can only be called once.
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
    /// let mut result = source.resize(Default::default()).await?;
    ///
    /// result.to_file("output.png").await?;
    /// println!("Image saved to output.png");
    /// # Ok::<(), tinify_rs::TinifyError>(())
    /// # });
    /// ```
    pub async fn to_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let bytes = self.to_buffer().await?;
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }

    /// Get compression count
    ///
    /// Returns the compression count statistics for the current API key this month.
    ///
    /// # Returns
    ///
    /// Returns `Some(count)` if the response header contains compression count information, otherwise returns `None`.
    pub fn compression_count(&self) -> Option<u32> {
        self.response
            .as_ref()?
            .headers()
            .get("Compression-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    /// Get image width
    ///
    /// Returns the width (in pixels) of the processed image.
    ///
    /// # Returns
    ///
    /// Returns `Some(width)` if the response header contains image width information, otherwise returns `None`.
    pub fn image_width(&self) -> Option<u32> {
        self.response
            .as_ref()?
            .headers()
            .get("Image-Width")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    /// Get image height
    ///
    /// Returns the height (in pixels) of the processed image.
    ///
    /// # Returns
    ///
    /// Returns `Some(height)` if the response header contains image height information, otherwise returns `None`.
    pub fn image_height(&self) -> Option<u32> {
        self.response
            .as_ref()?
            .headers()
            .get("Image-Height")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    /// Get content type
    ///
    /// Returns the MIME type of the response (such as "image/jpeg", "image/png", etc.).
    ///
    /// # Returns
    ///
    /// Returns `Some(content_type)` if the response header contains content type information, otherwise returns `None`.
    pub fn content_type(&self) -> Option<String> {
        self.response
            .as_ref()?
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
    }

    /// Get content length
    ///
    /// Returns the byte size of the response content.
    ///
    /// # Returns
    ///
    /// Returns `Some(length)` if the response header contains content length information, otherwise returns `None`.
    pub fn content_length(&self) -> Option<u64> {
        self.response
            .as_ref()?
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }
}
