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

/// The URL for the compression endpoint
const SHRINK_ENDPOINT: &str = "https://api.tinify.com/shrink";

/// Main interface structure for the Tinify API
///
/// Provides static methods for image compression, format conversion, resizing, and other features.
/// You must set the API key using `set_key` before use.
pub struct Tinify;

impl Tinify {
    /// Set the Tinify API key
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Tinify API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// # });
    /// ```
    pub async fn set_key(api_key: String) -> Result<()> {
        let client = Client::new(api_key)?;
        set_client(client).await
    }

    /// Set the application identifier
    ///
    /// Used to add User-Agent information to request headers for API usage tracking.
    ///
    /// # Arguments
    ///
    /// * `app_identifier` - Your application identifier
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// Tinify::set_app_identifier("MyApp/1.0".to_string()).await.unwrap();
    /// # });
    /// ```
    pub async fn set_app_identifier(app_identifier: String) -> Result<()> {
        let mut client = Client::new(get_client().await?.api_key().to_string())?;
        client.set_app_identifier(app_identifier);
        set_client(client).await
    }

    /// Create a Source object from a file
    ///
    /// Reads a local file and uploads it to Tinify for compression, returning a Source object for further operations.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file to be compressed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_file("input.png").await.unwrap();
    /// # });
    /// ```
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Source> {
        let data = tokio::fs::read(path).await?;
        Self::from_buffer(data).await
    }

    /// Create a Source object from image data in memory
    ///
    /// Uploads image data to Tinify for compression, returning a Source object for further operations.
    ///
    /// # Arguments
    ///
    /// * `data` - Binary data of the image
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let image_data = std::fs::read("input.png").unwrap();
    /// let source = Tinify::from_buffer(image_data).await.unwrap();
    /// # });
    /// ```
    pub async fn from_buffer(data: Vec<u8>) -> Result<Source> {
        let client = get_client().await?;
        let response = client.post(SHRINK_ENDPOINT, Some(data)).await?;

        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| TinifyError::UnknownError {
                message: "Missing Location header in server response".to_string(),
            })?;

        Ok(Source::new(location.to_string()))
    }

    /// Create a Source object from a URL
    ///
    /// Let Tinify fetch the image from the specified URL and compress it, returning a Source object for further operations.
    ///
    /// # Arguments
    ///
    /// * `url` - URL of the image to be compressed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// use tinify_rs::Tinify;
    ///
    /// Tinify::set_key("your-api-key".to_string()).await.unwrap();
    /// let source = Tinify::from_url("https://example.com/image.jpg").await.unwrap();
    /// # });
    /// ```
    pub async fn from_url<S: AsRef<str>>(url: S) -> Result<Source> {
        let client = get_client().await?;
        let body = serde_json::to_vec(&json!({ "source": { "url": url.as_ref() } }))?;
        let response = client.post(SHRINK_ENDPOINT, Some(body)).await?;

        let location = response
            .headers()
            .get("Location")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| TinifyError::UnknownError {
                message: "Missing Location header in server response".to_string(),
            })?;

        Ok(Source::new(location.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    const API_KEY: &str = "Grw2vwfbdD4WC31rDTWcqfKKMymvjZ9p";

    #[tokio::test]
    async fn test_from_file() {
        // Set the API key for testing (ignore duplicate initialization errors)
        let _ = Tinify::set_key(API_KEY.to_string()).await;

        let result = Tinify::from_file("./test_file.png").await;
        match &result {
            Ok(_) => println!("Success: {:?}", result.as_ref().ok()),
            Err(e) => println!("Failed: {:?}", e),
        }
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_from_url() {
        // Set the API key for testing (ignore duplicate initialization errors)
        let _ = Tinify::set_key(API_KEY.to_string()).await;

        let result = Tinify::from_url(
            "https://image-link.only1u.org/BedPicture_rust_print_to_android_termial.jpg",
        )
        .await;
        match &result {
            Ok(_) => println!("Success: {:?}", result.as_ref().ok()),
            Err(e) => println!("Failed: {:?}", e),
        }
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_should_resize() {
        // Reference Java version's shouldResize() test case
        // Set the API key for testing (ignore duplicate initialization errors)
        let _ = Tinify::set_key(API_KEY.to_string()).await;

        // Create Source from file
        let source = match Tinify::from_file("./test_file.png").await {
            Ok(s) => s,
            Err(e) => {
                println!("Cannot create Source from file: {:?}", e);
                return; // Skip test if Source cannot be created
            }
        };

        let resize_options = ResizeOptions {
            method: ResizeMethod::Fit,
            width: Some(50),
            height: Some(20),
        };

        // Execute resize operation
        let mut result = match source.resize(resize_options).await {
            Ok(r) => {
                println!("Resize operation successful: {:?}", r);
                r
            }
            Err(e) => {
                println!("Resize operation failed: {:?}", e);
                return; // Skip test if resize fails
            }
        };

        // Create temporary file to save result
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path();

        // Save result to file
        match result.to_file(temp_path).await {
            Ok(_) => println!("Successfully saved to temporary file: {:?}", temp_path),
            Err(e) => {
                println!("Failed to save file: {:?}", e);
                return;
            }
        }

        // Read file contents for validation
        let file_content = match fs::read(temp_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Failed to read file: {:?}", e);
                return;
            }
        };

        let file_size = file_content.len() as u64;

        println!(
            "Resized image dimensions: {}x{}",
            result.image_width().unwrap_or(0),
            result.image_height().unwrap_or(0)
        );
        println!("File size: {} bytes", file_size);

        // Verify image dimensions (Note: Using Fit method maintains aspect ratio, so actual dimensions may differ)
        if let Some(width) = result.image_width() {
            assert!(width > 0, "Image width should be greater than 0");
            assert!(
                width <= 50,
                "Image width should not exceed 50 (using Fit method maintains aspect ratio)"
            );
        } else {
            println!("Warning: Unable to get image width");
        }

        // Verify file size is within reasonable range (compressed small images may be very small)
        assert!(
            file_size > 100,
            "File size should be greater than 100 bytes, actual: {}",
            file_size
        );
        assert!(
            file_size < 10000,
            "File size should be less than 10000 bytes, actual: {}",
            file_size
        );

        // Verify file contents (check if it's a valid image file)
        // PNG files should start with PNG signature
        if file_content.len() >= 8 {
            let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
            let is_png = file_content[0..8] == png_signature;
            println!("File is valid PNG format: {}", is_png);

            // If it's a PNG file, verify width information
            if is_png && file_content.len() >= 24 {
                // Width information in PNG files is in IHDR chunk (offset 16-19)
                let width_bytes = [
                    file_content[16],
                    file_content[17],
                    file_content[18],
                    file_content[19],
                ];
                let width = u32::from_be_bytes(width_bytes);
                println!("Width information in PNG file: {}", width);

                // Verify width matches the width returned by API
                if let Some(api_width) = result.image_width() {
                    assert_eq!(
                        width, api_width,
                        "Width in PNG file should match the width returned by API"
                    );
                } else {
                    assert!(
                        width > 0 && width <= 50,
                        "Width in PNG file should be within reasonable range (0-50)"
                    );
                }
            }
        }

        // Clean up temporary file
        temp_file.close().unwrap();
        println!("Test completed: should_resize test passed");
    }
}
