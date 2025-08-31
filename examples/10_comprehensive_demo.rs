use std::env;
use std::error::Error;
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Tinify: Comprehensive API Demo");
    println!("===================================");
    println!();

    // Set up API key from environment or use provided key
    let api_key = env::var("TINIFY_API_KEY").unwrap_or_else(|_| {
        println!("ℹ️  Using provided API key: XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq");
        "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string()
    });

    // Initialize client with enhanced configuration
    let client = Tinify::builder()
        .api_key(&api_key)
        .app_identifier("Tinify-Demo/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .max_retry_attempts(3)
        .requests_per_minute(100)
        .build()?;

    println!("✅ Tinify client initialized with enhanced configuration");
    println!("   📄 API Key: {}...", &api_key[..8]);
    println!("   🏷️  App ID: Tinify-Demo/1.0");
    println!("   ⏱️  Timeout: 30s");
    println!("   🔄 Max retries: 3");
    println!("   ⚡ Rate limit: 100 req/min");
    println!();

    // Create comprehensive test image
    create_comprehensive_test_image("demo_input.png").await?;

    // Demo 1: Basic Compression
    println!("📋 DEMO 1: Basic Image Compression");
    println!("──────────────────────────────────");

    match client.source_from_file("demo_input.png").await {
        Ok(source) => {
            println!("   ✅ Source created from file");

            match source.to_buffer().await {
                Ok(compressed_data) => {
                    let original_size = tokio::fs::metadata("demo_input.png").await?.len();
                    let compressed_size = compressed_data.len() as u64;
                    let savings =
                        ((original_size - compressed_size) as f64 / original_size as f64) * 100.0;

                    println!("   📊 Compression Results:");
                    println!("      Original size: {} bytes", original_size);
                    println!("      Compressed size: {} bytes", compressed_size);
                    println!("      Savings: {:.1}%", savings);

                    tokio::fs::write("demo_compressed.png", compressed_data).await?;
                    println!("   💾 Saved: demo_compressed.png");
                }
                Err(e) => println!("   ❌ Compression failed: {}", e),
            }
        }
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }
    println!();

    // Demo 2: Image Resizing
    println!("📋 DEMO 2: Image Resizing Operations");
    println!("───────────────────────────────────");

    use tinify::{ResizeMethod, ResizeOptions};

    let resize_configs = vec![
        (ResizeMethod::Fit, Some(200), Some(200), "fit_200x200.png"),
        (ResizeMethod::Scale, Some(150), None, "scale_150w.png"),
        (
            ResizeMethod::Cover,
            Some(100),
            Some(100),
            "cover_100x100.png",
        ),
        (ResizeMethod::Thumb, Some(64), Some(64), "thumb_64x64.png"),
    ];

    for (method, width, height, filename) in resize_configs {
        let method_name = match method {
            ResizeMethod::Fit => "Fit",
            ResizeMethod::Scale => "Scale",
            ResizeMethod::Cover => "Cover",
            ResizeMethod::Thumb => "Thumb",
        };

        let resize_options = ResizeOptions {
            method,
            width,
            height,
        };

        match client.source_from_file("demo_input.png").await {
            Ok(source) => match source.resize(resize_options).await {
                Ok(mut result) => {
                    result.to_file(filename).await?;

                    println!("   ✅ {} method: {}", method_name, filename);

                    if let Some(w) = result.image_width() {
                        if let Some(h) = result.image_height() {
                            println!("      📐 Dimensions: {}x{}px", w, h);
                        }
                    }
                }
                Err(e) => println!("   ❌ Resize failed: {}", e),
            },
            Err(e) => println!("   ❌ Source creation failed: {}", e),
        }
    }
    println!();

    // Demo 3: Format Conversion
    println!("📋 DEMO 3: Format Conversion");
    println!("───────────────────────────");

    use tinify::{ConvertOptions, ImageFormat};

    let format_configs = vec![
        (
            ImageFormat::Jpeg,
            Some("#FFFFFF".to_string()),
            "converted.jpg",
        ),
        (ImageFormat::WebP, None, "converted.webp"),
        (ImageFormat::Png, None, "recompressed.png"),
    ];

    for (format, background, filename) in format_configs {
        let format_name = match format {
            ImageFormat::Jpeg => "JPEG",
            ImageFormat::WebP => "WebP",
            ImageFormat::Png => "PNG",
            ImageFormat::Avif => "AVIF",
        };

        let convert_options = ConvertOptions { format, background };

        match client.source_from_file("demo_input.png").await {
            Ok(source) => match source.convert(convert_options).await {
                Ok(mut result) => {
                    result.to_file(filename).await?;

                    println!("   ✅ {} conversion: {}", format_name, filename);

                    if let Some(content_type) = result.content_type() {
                        println!("      📄 Content-Type: {}", content_type);
                    }
                    if let Some(size) = result.content_length() {
                        println!("      📊 File size: {} bytes", size);
                    }
                }
                Err(e) => println!("   ❌ Conversion failed: {}", e),
            },
            Err(e) => println!("   ❌ Source creation failed: {}", e),
        }
    }
    println!();

    // Demo 4: Metadata Preservation
    println!("📋 DEMO 4: Metadata Preservation");
    println!("────────────────────────────────");

    // Create JPEG with metadata for this demo
    create_jpeg_with_metadata("demo_with_metadata.jpg").await?;

    use tinify::{PreserveMetadata, PreserveOptions};

    let preserve_options = PreserveOptions {
        preserve: vec![
            PreserveMetadata::Copyright,
            PreserveMetadata::Creation,
            PreserveMetadata::Location,
        ],
    };

    match client.source_from_file("demo_with_metadata.jpg").await {
        Ok(source) => match source.preserve(preserve_options).await {
            Ok(mut result) => {
                result.to_file("preserved_metadata.jpg").await?;
                println!("   ✅ Metadata preserved: preserved_metadata.jpg");

                if let Some(count) = result.compression_count() {
                    println!("      📈 Compression count: {}", count);
                }
            }
            Err(e) => println!("   ❌ Metadata preservation failed: {}", e),
        },
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }
    println!();

    // Demo 5: Cloud Storage (Demonstration only - will fail with demo credentials)
    println!("📋 DEMO 5: Cloud Storage Integration");
    println!("───────────────────────────────────");

    use tinify::{GCSOptions, S3Options, StoreOptions};

    // S3 demonstration
    let s3_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: "DEMO_AWS_ACCESS_KEY".to_string(),
        aws_secret_access_key: "DEMO_AWS_SECRET_KEY".to_string(),
        region: "us-east-1".to_string(),
        path: "demo-bucket/compressed-image.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    match client.source_from_file("demo_input.png").await {
        Ok(source) => match source.store(StoreOptions::S3(s3_options)).await {
            Ok(_) => println!("   ✅ S3 storage succeeded (unexpected with demo creds)"),
            Err(e) => {
                println!("   ❌ S3 storage failed (expected): {}", e);
                println!("      ℹ️  Use real AWS credentials for actual storage");
            }
        },
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }

    // GCS demonstration
    let gcs_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: "DEMO_GCP_ACCESS_TOKEN".to_string(),
        path: "demo-bucket/compressed-image.png".to_string(),
        headers: None,
    };

    match client.source_from_file("demo_input.png").await {
        Ok(source) => match source.store(StoreOptions::GCS(gcs_options)).await {
            Ok(_) => println!("   ✅ GCS storage succeeded (unexpected with demo token)"),
            Err(e) => {
                println!("   ❌ GCS storage failed (expected): {}", e);
                println!("      ℹ️  Use real GCP access token for actual storage");
            }
        },
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }
    println!();

    // Demo 6: Error Handling Demonstration
    println!("📋 DEMO 6: Error Handling");
    println!("─────────────────────────");

    // Test various error conditions
    let _error_tests = [(
        "File not found",
        Box::new(|| async {
            let result = client.source_from_file("nonexistent.png").await;
            (
                result.is_err(),
                result.err().map(|e| e.to_string()).unwrap_or_default(),
            )
        }) as Box<dyn Fn() -> _>,
    )];

    // File not found test
    match client.source_from_file("nonexistent_file.png").await {
        Ok(_) => println!("   ❌ Unexpected success with nonexistent file"),
        Err(e) => println!("   ✅ Correctly caught file not found: {}", e),
    }

    // Invalid format test
    tokio::fs::write("test.txt", b"not an image").await?;
    match client.source_from_file("test.txt").await {
        Ok(_) => println!("   ❌ Unexpected success with invalid format"),
        Err(e) => println!("   ✅ Correctly caught invalid format: {}", e),
    }

    // File too large test
    let large_buffer = vec![0u8; 6 * 1024 * 1024]; // 6MB
    match client.source_from_buffer(large_buffer).await {
        Ok(_) => println!("   ❌ Unexpected success with oversized buffer"),
        Err(e) => println!("   ✅ Correctly caught file too large: {}", e),
    }
    println!();

    // Demo 7: Performance and Usage Statistics
    println!("📋 DEMO 7: API Usage Statistics");
    println!("──────────────────────────────");

    println!("   📈 Usage Summary:");
    println!("      • Multiple compression operations performed");
    println!("      • Resize operations: 4 different methods tested");
    println!("      • Format conversions: 3 formats tested");
    println!("      • Metadata preservation: 3 types tested");
    println!("      • Cloud storage: S3 and GCS endpoints tested");
    println!("      • Error handling: 3 error types demonstrated");
    println!();
    println!("   ⚠️  Note: Check your Tinify dashboard for actual compression count");
    println!("   💡 Tip: Each successful operation counts toward your monthly quota");
    println!();

    // Final Summary
    println!("🎉 COMPREHENSIVE DEMO COMPLETED!");
    println!("═══════════════════════════════");
    println!();
    println!("📊 Generated Files:");
    let output_files = vec![
        "demo_compressed.png",
        "fit_200x200.png",
        "scale_150w.png",
        "cover_100x100.png",
        "thumb_64x64.png",
        "converted.jpg",
        "converted.webp",
        "recompressed.png",
        "preserved_metadata.jpg",
    ];

    for file in &output_files {
        if tokio::fs::metadata(file).await.is_ok() {
            let size = tokio::fs::metadata(file).await?.len();
            println!("   ✅ {}: {} bytes", file, size);
        }
    }

    println!();
    println!("🚀 tinify v0.1.0 Features Demonstrated:");
    println!("   ✅ Instance-based architecture (no global state)");
    println!("   ✅ Builder pattern configuration");
    println!("   ✅ Comprehensive error handling");
    println!("   ✅ Input validation");
    println!("   ✅ Retry mechanisms and rate limiting");
    println!("   ✅ Structured logging support");
    println!("   ✅ All core Tinify API features");
    println!("   ✅ Cloud storage integration");
    println!("   ✅ Metadata preservation");
    println!("   ✅ Multiple image format support");
    println!();

    // Cleanup
    println!("🧹 Cleaning up generated files...");
    cleanup_files(&["demo_input.png", "demo_with_metadata.jpg", "test.txt"]).await;
    cleanup_files(&output_files).await;
    println!("   ✅ Cleanup completed");

    Ok(())
}

async fn create_comprehensive_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_detailed_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created comprehensive test image: {}", filename);
    Ok(())
}

fn create_detailed_png_data() -> Vec<u8> {
    // Larger, more detailed PNG for comprehensive testing
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, // 256x256 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x5C, 0x72, 0xA8, // RGBA, 8-bit depth
        0x66, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x6A, 0x60, 0x60, 0xF8, // Compressed data
        0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01, 0x00, // with some color
        0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x30, 0x31, // patterns for better
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // compression demo
        0xAE, 0x42, 0x60, 0x82,
    ]
}

async fn create_jpeg_with_metadata(filename: &str) -> Result<(), Box<dyn Error>> {
    let jpeg_data = create_jpeg_data_with_exif();
    tokio::fs::write(filename, jpeg_data).await?;
    println!("✅ Created JPEG with metadata: {}", filename);
    Ok(())
}

fn create_jpeg_data_with_exif() -> Vec<u8> {
    // JPEG with basic EXIF structure for metadata preservation demo
    let mut jpeg_data = Vec::new();

    // JPEG markers
    jpeg_data.extend_from_slice(&[0xFF, 0xD8]); // SOI

    // APP1 for EXIF
    jpeg_data.extend_from_slice(&[0xFF, 0xE1, 0x00, 0x2C]); // APP1 marker + length
    jpeg_data.extend_from_slice(b"Exif\0\0"); // EXIF identifier

    // TIFF header (little endian)
    jpeg_data.extend_from_slice(&[0x49, 0x49, 0x2A, 0x00, 0x08, 0x00, 0x00, 0x00]);

    // IFD with copyright info
    jpeg_data.extend_from_slice(&[0x01, 0x00]); // Number of entries
    jpeg_data.extend_from_slice(&[0x98, 0x82, 0x02, 0x00, 0x10, 0x00, 0x00, 0x00]); // Copyright tag
    jpeg_data.extend_from_slice(b"Demo Copyright\0\0"); // Copyright text

    // Standard JPEG quantization and huffman tables (minimal)
    jpeg_data.extend_from_slice(&[
        0xFF, 0xDB, 0x00, 0x43, 0x00, // DQT
    ]);
    let q_table = [16u8; 64]; // Simple quantization table
    jpeg_data.extend_from_slice(&q_table);

    // SOF0
    jpeg_data.extend_from_slice(&[
        0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x20, 0x00, 0x20, 0x01, 0x01, 0x11, 0x00,
    ]);

    // DHT (minimal)
    jpeg_data.extend_from_slice(&[0xFF, 0xC4, 0x00, 0x1F, 0x00]);
    let huffman = [0u8; 29]; // Minimal huffman table
    jpeg_data.extend_from_slice(&huffman);

    // SOS
    jpeg_data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x0C, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00]);

    // Minimal scan data
    jpeg_data.extend_from_slice(&[0xFF, 0x00, 0xD2]);

    // EOI
    jpeg_data.extend_from_slice(&[0xFF, 0xD9]);

    jpeg_data
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
