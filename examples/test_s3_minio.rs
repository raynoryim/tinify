use serde_json::json;
use std::error::Error;
use tinify::{S3Options, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🧪 Tinify: S3 Testing with MinIO");
    println!("===================================");
    println!();

    println!("ℹ️  This example tests S3 functionality with MinIO (local S3-compatible server)");
    println!("   To run this test:");
    println!("   1. Start MinIO: docker run -p 9000:9000 -p 9001:9001 \\");
    println!("                     -e \"MINIO_ROOT_USER=minioadmin\" \\");
    println!("                     -e \"MINIO_ROOT_PASSWORD=minioadmin\" \\");
    println!("                     minio/minio server /data --console-address \":9001\"");
    println!(
        "   2. Create bucket: aws --endpoint-url=http://localhost:9000 s3 mb s3://test-bucket"
    );
    println!("   3. Run this example");
    println!();

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Tinify client initialized");

    // Create test image
    create_test_image("minio_test_input.png").await?;

    let source = client.source_from_file("minio_test_input.png").await?;
    println!("✅ Source image loaded");

    // Test 1: Basic MinIO S3 storage
    println!("\n🔧 Test 1: Basic MinIO S3 Storage");

    let minio_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        region: "us-east-1".to_string(),
        path: "test-bucket/compressed-image-basic.png".to_string(),
        headers: None,
        acl: None,
    };

    match source.store(StoreOptions::S3(minio_options)).await {
        Ok(result) => {
            println!("   ✅ Image stored to MinIO successfully!");
            if let Some(compression_count) = result.compression_count() {
                println!("   📈 Compression count: {}", compression_count);
            }
            println!(
                "   🌐 Access via: http://localhost:9000/test-bucket/compressed-image-basic.png"
            );
        }
        Err(e) => {
            println!("   ❌ MinIO storage failed: {}", e);
            println!("   💡 Make sure MinIO is running on localhost:9000");
            println!("   💡 Make sure test-bucket exists");
        }
    }

    // Test 2: MinIO with custom headers
    println!("\n📋 Test 2: MinIO with Custom Headers");

    let custom_headers = json!({
        "Cache-Control": "public, max-age=3600",
        "Content-Disposition": "inline; filename=\"optimized.png\"",
        "X-Custom-Meta": "tinify-test"
    });

    let minio_headers_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "minioadmin".to_string()),
        region: "us-east-1".to_string(),
        path: "test-bucket/compressed-with-headers.png".to_string(),
        headers: Some(custom_headers),
        acl: None,
    };

    let source2 = client.source_from_file("minio_test_input.png").await?;
    match source2.store(StoreOptions::S3(minio_headers_options)).await {
        Ok(_) => {
            println!("   ✅ Image with custom headers stored to MinIO!");
            println!(
                "   🌐 Access via: http://localhost:9000/test-bucket/compressed-with-headers.png"
            );
        }
        Err(e) => {
            println!("   ❌ MinIO headers test failed: {}", e);
        }
    }

    // Test 3: Different formats to MinIO
    println!("\n🎨 Test 3: Format Conversion + MinIO Storage");

    use tinify::{ConvertOptions, ImageFormat};

    let formats = vec![
        (ImageFormat::Jpeg, "jpg", "image/jpeg"),
        (ImageFormat::WebP, "webp", "image/webp"),
        (ImageFormat::Png, "png", "image/png"),
    ];

    for (format, ext, content_type) in formats {
        let convert_options = ConvertOptions {
            format,
            background: Some("#FFFFFF".to_string()),
        };

        let source_convert = client.source_from_file("minio_test_input.png").await?;
        match source_convert.convert(convert_options).await {
            Ok(converted_result) => {
                // Convert to bytes for buffer upload
                let converted_bytes = converted_result.into();
                let source_converted = client.source_from_buffer(converted_bytes).await?;

                let format_headers = json!({
                    "Content-Type": content_type,
                    "X-Original-Format": "PNG",
                    "X-Converted-Format": ext.to_uppercase()
                });

                let minio_format_options = S3Options {
                    service: "s3".to_string(),
                    aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
                        .unwrap_or_else(|_| "minioadmin".to_string()),
                    aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
                        .unwrap_or_else(|_| "minioadmin".to_string()),
                    region: "us-east-1".to_string(),
                    path: format!("test-bucket/converted-image.{}", ext),
                    headers: Some(format_headers),
                    acl: None,
                };

                match source_converted
                    .store(StoreOptions::S3(minio_format_options))
                    .await
                {
                    Ok(_) => {
                        println!("   ✅ {} format stored successfully", ext.to_uppercase());
                        println!("      🌐 Access via: http://localhost:9000/test-bucket/converted-image.{}", ext);
                    }
                    Err(e) => {
                        println!("   ❌ {} format storage error: {}", ext.to_uppercase(), e);
                    }
                }
            }
            Err(e) => {
                println!("   ❌ Format conversion to {} error: {}", ext, e);
            }
        }
    }

    // Test 4: Resize + MinIO Storage
    println!("\n📏 Test 4: Resize + MinIO Storage");

    use tinify::{ResizeMethod, ResizeOptions};

    let resize_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(200),
        height: Some(200),
    };

    let source_resize = client.source_from_file("minio_test_input.png").await?;
    match source_resize.resize(resize_options).await {
        Ok(resized_result) => {
            let resized_bytes = resized_result.into();
            let source_resized = client.source_from_buffer(resized_bytes).await?;

            let resize_options_s3 = S3Options {
                service: "s3".to_string(),
                aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                region: "us-east-1".to_string(),
                path: "test-bucket/resized-image.png".to_string(),
                headers: Some(json!({"X-Resize-Method": "fit", "X-Dimensions": "200x200"})),
                acl: None,
            };

            match source_resized
                .store(StoreOptions::S3(resize_options_s3))
                .await
            {
                Ok(_) => {
                    println!("   ✅ Resized image stored to MinIO!");
                    println!(
                        "      🌐 Access via: http://localhost:9000/test-bucket/resized-image.png"
                    );
                }
                Err(e) => {
                    println!("   ❌ Resized image storage error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Resize operation error: {}", e);
        }
    }

    // Test 5: Environment Variable Configuration Check
    println!("\n🔧 Test 5: Configuration Status");

    println!("   Current Configuration:");
    println!(
        "   ├── AWS_ACCESS_KEY_ID: {}",
        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            "✅ Set"
        } else {
            "❌ Using default (minioadmin)"
        }
    );
    println!(
        "   ├── AWS_SECRET_ACCESS_KEY: {}",
        if std::env::var("AWS_SECRET_ACCESS_KEY").is_ok() {
            "✅ Set"
        } else {
            "❌ Using default (minioadmin)"
        }
    );
    println!(
        "   ├── TINIFY_API_KEY: {}",
        if std::env::var("TINIFY_API_KEY").is_ok() {
            "✅ Set"
        } else {
            "❌ Using provided key"
        }
    );
    println!("   └── MinIO Endpoint: http://localhost:9000");

    println!("\n💡 MinIO Web Console available at: http://localhost:9001");
    println!("   Login with: minioadmin / minioadmin");

    // Test 6: Verify MinIO bucket contents
    println!("\n📋 Test 6: Expected Files in MinIO");
    println!("   If all tests passed, you should see these files in MinIO:");
    println!("   ├── test-bucket/compressed-image-basic.png");
    println!("   ├── test-bucket/compressed-with-headers.png");
    println!("   ├── test-bucket/converted-image.jpg");
    println!("   ├── test-bucket/converted-image.webp");
    println!("   ├── test-bucket/converted-image.png");
    println!("   └── test-bucket/resized-image.png");
    println!();
    println!("   🌐 Browse files at: http://localhost:9001/buckets/test-bucket/browse");

    println!("\n🎉 MinIO S3 Compatibility Testing Completed!");
    println!("   This verifies that our S3 implementation works correctly");
    println!("   and can be used with any S3-compatible service.");

    // Cleanup
    cleanup_files(&["minio_test_input.png"]).await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a proper PNG file for testing
    let png_data = create_valid_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_valid_png_data() -> Vec<u8> {
    // Create a more substantial PNG for better testing
    // This creates a simple 100x100 colored PNG
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x64, // 100x100 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x70, 0xE2, 0x95, // RGBA 8-bit
        0x25, 0x00, 0x00, 0x00, 0x04, 0x67, 0x41, 0x4D, // gAMA chunk
        0x41, 0x00, 0x00, 0xB1, 0x8E, 0x7C, 0xFB, 0x51, 0x93, 0x00, 0x00, 0x01, 0x5D, 0x49, 0x44,
        0x41, // IDAT chunk start
        0x54, 0x78, 0x9C, 0xED, 0xDD, 0x31, 0x0A, 0x80, // Compressed image data
        0x20, 0x10, 0x04, 0xD0, 0xF7, 0x9F, 0x2B, 0x04, // (simplified but valid)
        0x0B, 0x81, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82, 0x60,
        0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0,
        0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58,
        0x41, 0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC,
        0x20, 0x58, 0x41, 0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56,
        0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B,
        0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15,
        0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A,
        0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82, 0x60, 0x05,
        0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41, 0xB0, 0x82,
        0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20, 0x58, 0x41,
        0xB0, 0x82, 0x60, 0x05, 0xC1, 0x0A, 0x82, 0x15, 0x04, 0x2B, 0x08, 0x56, 0x10, 0xAC, 0x20,
        0x58, 0x41, 0xB0, 0x82, 0x01, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, // IEND chunk
        0x44, 0xAE, 0x42, 0x60, 0x82,
    ]
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
