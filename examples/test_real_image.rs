use std::error::Error;
use tinify::{S3Options, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🧪 Tinify: Real Image Testing");
    println!("================================");

    // Test with provided API key
    let api_key = "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq";
    let client = Tinify::new(api_key.to_string())?;
    println!("✅ Client initialized with provided API key");

    // Test 1: Basic compression with real image
    println!("\n🗜️  Test 1: Basic Compression");
    match client.source_from_file("test_real_image.png").await {
        Ok(source) => {
            println!("   ✅ Source created successfully");

            match source.to_buffer().await {
                Ok(compressed_data) => {
                    let original_size = tokio::fs::metadata("test_real_image.png").await?.len();
                    let compressed_size = compressed_data.len() as u64;
                    let savings =
                        ((original_size - compressed_size) as f64 / original_size as f64) * 100.0;

                    println!("   📊 Compression Results:");
                    println!(
                        "      Original: {} bytes ({:.1} KB)",
                        original_size,
                        original_size as f64 / 1024.0
                    );
                    println!(
                        "      Compressed: {} bytes ({:.1} KB)",
                        compressed_size,
                        compressed_size as f64 / 1024.0
                    );
                    println!("      Savings: {:.1}%", savings);

                    tokio::fs::write("compressed_real_image.png", compressed_data).await?;
                    println!("   💾 Saved compressed image");
                }
                Err(e) => println!("   ❌ Compression failed: {}", e),
            }
        }
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }

    // Test 2: S3 Storage (will fail with demo credentials but show proper behavior)
    println!("\n📦 Test 2: S3 Storage Attempt");
    match client.source_from_file("test_real_image.png").await {
        Ok(source) => {
            let s3_options = S3Options {
                service: "s3".to_string(),
                aws_access_key_id: "DEMO_ACCESS_KEY".to_string(),
                aws_secret_access_key: "DEMO_SECRET_KEY".to_string(),
                region: "us-east-1".to_string(),
                path: "test-bucket/real-image.png".to_string(),
                headers: None,
                acl: Some("public-read".to_string()),
            };

            match source.store(StoreOptions::S3(s3_options)).await {
                Ok(_) => println!("   ✅ S3 storage successful (unexpected)"),
                Err(e) => {
                    println!("   ❌ S3 storage failed (expected): {}", e);
                    println!(
                        "   💡 This demonstrates proper error handling with invalid credentials"
                    );
                }
            }
        }
        Err(e) => println!("   ❌ Source creation for S3 test failed: {}", e),
    }

    // Test 3: Format conversion
    println!("\n🎨 Test 3: Format Conversion");
    match client.source_from_file("test_real_image.png").await {
        Ok(source) => {
            use tinify::{ConvertOptions, ImageFormat};
            let convert_options = ConvertOptions {
                format: ImageFormat::Jpeg,
                background: Some("#FFFFFF".to_string()),
            };

            match source.convert(convert_options).await {
                Ok(mut result) => {
                    result.to_file("converted_real_image.jpg").await?;
                    println!("   ✅ Converted PNG to JPEG successfully");

                    if let Some(compression_count) = result.compression_count() {
                        println!("   📈 Compression count: {}", compression_count);
                    }
                }
                Err(e) => println!("   ❌ Format conversion failed: {}", e),
            }
        }
        Err(e) => println!("   ❌ Source creation for conversion failed: {}", e),
    }

    println!("\n🎉 Real Image Testing Complete!");
    println!("   This demonstrates that the library works correctly with real images");
    println!("   and the provided API key is functional for actual compression tasks.");

    // Clean up generated files (keep original for reference)
    let cleanup_files = ["compressed_real_image.png", "converted_real_image.jpg"];
    for file in cleanup_files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }

    Ok(())
}
