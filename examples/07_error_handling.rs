use std::error::Error;
use tinify::{Tinify, TinifyError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("⚠️  Tinify: Error Handling Example");
    println!("===================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Example 1: Invalid API key error
    println!("\n🔑 Example 1: Invalid API key error");
    match Tinify::new("invalid-api-key".to_string()) {
        Ok(_) => println!("   ❌ Unexpected success with invalid key"),
        Err(e) => {
            println!("   ✅ Correctly caught invalid API key error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::InvalidApiKey => println!("      Type: InvalidApiKey"),
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 2: File not found error
    println!("\n📁 Example 2: File not found error");
    match client.source_from_file("nonexistent_file.png").await {
        Ok(_) => println!("   ❌ Unexpected success with nonexistent file"),
        Err(e) => {
            println!("   ✅ Correctly caught file not found error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::FileNotFound { path } => {
                    println!("      Type: FileNotFound");
                    println!("      Path: {:?}", path);
                }
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 3: Unsupported format error
    println!("\n🖼️  Example 3: Unsupported format error");

    // Create a test file with unsupported extension
    create_test_file_with_extension("test.txt", b"This is not an image").await?;

    match client.source_from_file("test.txt").await {
        Ok(_) => println!("   ❌ Unexpected success with unsupported format"),
        Err(e) => {
            println!("   ✅ Correctly caught unsupported format error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::UnsupportedFormat { format } => {
                    println!("      Type: UnsupportedFormat");
                    println!("      Format: {}", format);
                }
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 4: File too large error
    println!("\n📊 Example 4: File too large error");

    // Create a buffer that's too large (> 5MB)
    let large_buffer = vec![0u8; 6 * 1024 * 1024]; // 6MB
    match client.source_from_buffer(large_buffer).await {
        Ok(_) => println!("   ❌ Unexpected success with oversized buffer"),
        Err(e) => {
            println!("   ✅ Correctly caught file too large error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::FileTooLarge { size, max_size } => {
                    println!("      Type: FileTooLarge");
                    println!(
                        "      Size: {} bytes ({:.1} MB)",
                        size,
                        size as f64 / 1024.0 / 1024.0
                    );
                    println!(
                        "      Max size: {} bytes ({:.1} MB)",
                        max_size,
                        max_size as f64 / 1024.0 / 1024.0
                    );
                }
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 5: Invalid URL error
    println!("\n🌐 Example 5: Invalid URL error");
    match client.source_from_url("not-a-valid-url").await {
        Ok(_) => println!("   ❌ Unexpected success with invalid URL"),
        Err(e) => {
            println!("   ✅ Correctly caught URL parse error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::UrlParseError(url_error) => {
                    println!("      Type: UrlParseError");
                    println!("      Details: {}", url_error);
                }
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 6: Invalid dimensions error
    println!("\n📏 Example 6: Invalid dimensions error");

    // First create a valid source
    create_test_image("resize_test.png").await?;
    let source = client.source_from_file("resize_test.png").await?;

    // Try invalid resize dimensions
    use tinify::{ResizeMethod, ResizeOptions};
    let invalid_resize = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(0), // Invalid: zero width
        height: Some(100),
    };

    match source.resize(invalid_resize).await {
        Ok(_) => println!("   ❌ Unexpected success with invalid dimensions"),
        Err(e) => {
            println!("   ✅ Correctly caught invalid dimensions error:");
            println!("      Error: {}", e);
            match e {
                TinifyError::InvalidDimensions { width, height } => {
                    println!("      Type: InvalidDimensions");
                    println!("      Width: {:?}", width);
                    println!("      Height: {:?}", height);
                }
                _ => println!("      Type: Other"),
            }
        }
    }

    // Example 7: Network/HTTP errors
    println!("\n🌐 Example 7: Network/HTTP errors simulation");

    // Try with a potentially problematic URL
    match client.source_from_url("https://httpstat.us/429").await {
        Ok(_) => println!("   ⚠️  Unexpected success (or 429 was handled by retry)"),
        Err(e) => {
            println!("   ✅ Network error caught:");
            println!("      Error: {}", e);
            match e {
                TinifyError::ConnectionError(_) => {
                    println!("      Type: ConnectionError");
                }
                TinifyError::AccountError { status, .. } => {
                    println!("      Type: AccountError (Status: {:?})", status);
                }
                TinifyError::ClientError { status, .. } => {
                    println!("      Type: ClientError (Status: {:?})", status);
                }
                TinifyError::ServerError { status, .. } => {
                    println!("      Type: ServerError (Status: {:?})", status);
                }
                _ => println!("      Type: Other - {}", e),
            }
        }
    }

    // Example 8: Builder pattern errors
    println!("\n🏗️  Example 8: Builder pattern errors");
    match Tinify::builder().build() {
        Ok(_) => println!("   ❌ Unexpected success without API key"),
        Err(e) => {
            println!("   ✅ Builder correctly requires API key:");
            println!("      Error: {}", e);
        }
    }

    // Example 9: Error recovery patterns
    println!("\n🔄 Example 9: Error recovery patterns");

    let problematic_files = vec![
        "nonexistent.png",
        "test.txt",        // Wrong format
        "resize_test.png", // This one should work
    ];

    let mut successful_count = 0;
    let mut failed_count = 0;

    for file in problematic_files {
        match client.source_from_file(file).await {
            Ok(source) => {
                successful_count += 1;
                println!("   ✅ Successfully processed: {}", file);

                // Try to save it
                let output_name = format!("recovered_{}", file);
                match source.to_file(&output_name).await {
                    Ok(_) => println!("      💾 Saved as: {}", output_name),
                    Err(e) => println!("      ❌ Save failed: {}", e),
                }
            }
            Err(e) => {
                failed_count += 1;
                println!("   ❌ Failed to process {}: {}", file, e);
            }
        }
    }

    println!(
        "   📊 Recovery summary: {} successful, {} failed",
        successful_count, failed_count
    );

    // Example 10: Comprehensive error matching
    println!("\n🎯 Example 10: Comprehensive error type demonstration");

    println!("   Testing: Missing API key");
    match Tinify::builder().build() {
        Ok(_) => println!("      ❌ Unexpected success"),
        Err(e) => {
            println!("      ✅ Error caught: {}", e);

            // Demonstrate error type classification
            match e {
                TinifyError::InvalidApiKey => println!("         🔑 Authentication issue"),
                TinifyError::FileNotFound { .. } => println!("         📁 File system issue"),
                TinifyError::UnsupportedFormat { .. } => println!("         🖼️  Format issue"),
                TinifyError::FileTooLarge { .. } => println!("         📊 Size issue"),
                TinifyError::InvalidDimensions { .. } => println!("         📏 Dimension issue"),
                TinifyError::ConnectionError(_) => println!("         🌐 Connection issue"),
                TinifyError::AccountError { .. } => println!("         💳 Account issue"),
                TinifyError::ClientError { .. } => println!("         📡 Client issue"),
                TinifyError::ServerError { .. } => println!("         🖥️  Server issue"),
                TinifyError::QuotaExceeded => println!("         💳 Quota issue"),
                TinifyError::RateLimitExceeded { .. } => println!("         ⏱️  Rate limit issue"),
                TinifyError::UrlParseError(_) => println!("         🔗 URL issue"),
                TinifyError::JsonError(_) => println!("         📄 JSON issue"),
                TinifyError::IoError(_) => println!("         💾 I/O issue"),
                TinifyError::UnknownError { .. } => println!("         ❓ Unknown issue"),
                TinifyError::ClientNotInitialized => println!("         🚫 Client not initialized"),
            }
        }
    }

    println!("\n🎉 Error handling examples completed!");
    println!("\nℹ️  Error Handling Best Practices:");
    println!("   • Always match on specific error types for proper handling");
    println!("   • Use error recovery patterns for batch operations");
    println!("   • Log errors with context for debugging");
    println!("   • Implement retry logic for transient network errors");
    println!("   • Validate inputs before API calls when possible");

    // Clean up test files
    cleanup_files(&["test.txt", "resize_test.png", "recovered_resize_test.png"]).await;

    Ok(())
}

async fn create_test_file_with_extension(
    filename: &str,
    content: &[u8],
) -> Result<(), Box<dyn Error>> {
    tokio::fs::write(filename, content).await?;
    println!("   Created test file: {}", filename);
    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_simple_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("   Created test image: {}", filename);
    Ok(())
}

fn create_simple_png_data() -> Vec<u8> {
    // Minimal valid PNG
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, // 8x8 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0xC4, 0x0F, 0x26, // RGBA format
        0x93, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x64, 0x60, 0x60, 0xF8, 0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x30, 0x31, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
        0x4E, 0x44, // IEND chunk
        0xAE, 0x42, 0x60, 0x82,
    ]
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
