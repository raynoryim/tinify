use serde_json::json;
use std::error::Error;
use tinify::{GCSOptions, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("☁️  Tinify: Saving to Google Cloud Storage Example");
    println!("==================================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create test image
    create_test_image("gcs_input.png").await?;

    let source = client.source_from_file("gcs_input.png").await?;
    println!("✅ Source image loaded");

    // Example 1: Basic Google Cloud Storage
    println!("\n☁️  Example 1: Basic Google Cloud Storage");
    let gcs_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: std::env::var("GCP_ACCESS_TOKEN")
            .unwrap_or_else(|_| "DEMO_GCP_ACCESS_TOKEN".to_string()),
        path: "my-gcs-bucket/images/compressed-image.png".to_string(),
        headers: None,
    };

    match source.store(StoreOptions::GCS(gcs_options)).await {
        Ok(result) => {
            println!("   ✅ Image stored to Google Cloud Storage successfully!");
            if let Some(compression_count) = result.compression_count() {
                println!("   📈 Compression count: {}", compression_count);
            }
        }
        Err(e) => {
            println!("   ❌ GCS storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 2: GCS storage with custom headers
    println!("\n📋 Example 2: GCS storage with custom headers");
    let custom_headers = json!({
        "Cache-Control": "public, max-age=86400",
        "Content-Type": "image/png",
        "Content-Language": "en",
        "X-Goog-Meta-Source": "tinify-example"
    });

    let gcs_headers_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: std::env::var("GCP_ACCESS_TOKEN")
            .unwrap_or_else(|_| "DEMO_GCP_ACCESS_TOKEN".to_string()),
        path: "my-gcs-bucket/assets/header-demo.png".to_string(),
        headers: Some(custom_headers),
    };

    let source2 = client.source_from_file("gcs_input.png").await?;
    match source2.store(StoreOptions::GCS(gcs_headers_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to GCS with custom headers!");
        }
        Err(e) => {
            println!("   ❌ GCS headers storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 3: Different GCS bucket structures
    println!("\n📁 Example 3: Different GCS bucket and path structures");
    let path_examples = vec![
        ("my-images-bucket", "simple-file.png"),
        ("project-assets", "images/thumbnails/thumb.png"),
        ("cdn-bucket", "static/2024/01/15/image.png"),
        ("user-content", "uploads/user-123/profile-pic.png"),
        ("backup-storage", "archive/processed/batch-001.png"),
    ];

    for (bucket_part, file_path) in path_examples {
        let full_path = format!("{}/{}", bucket_part, file_path);
        let gcs_path_options = GCSOptions {
            service: "gcs".to_string(),
            gcp_access_token: "DEMO_GCP_ACCESS_TOKEN".to_string(),
            path: full_path.clone(),
            headers: None,
        };

        let source_path = client.source_from_file("gcs_input.png").await?;
        match source_path.store(StoreOptions::GCS(gcs_path_options)).await {
            Ok(_) => {
                println!("   ✅ Path structure '{}' accepted", full_path);
            }
            Err(e) => {
                println!("   ❌ Path '{}' error: {}", full_path, e);
            }
        }
    }

    // Example 4: GCS with different metadata headers
    println!("\n🏷️  Example 4: GCS with metadata and caching headers");
    let metadata_headers = json!({
        "Cache-Control": "public, max-age=31536000, immutable",
        "X-Goog-Meta-Project": "tinify-rust-demo",
        "X-Goog-Meta-Version": "1.0",
        "X-Goog-Meta-Compressed-At": "2024-01-15T10:30:00Z",
        "X-Goog-Meta-Original-Size": "1024000",
        "Content-Disposition": "inline; filename=\"optimized.png\""
    });

    let gcs_metadata_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: std::env::var("GCP_ACCESS_TOKEN")
            .unwrap_or_else(|_| "DEMO_GCP_ACCESS_TOKEN".to_string()),
        path: "my-metadata-bucket/processed/metadata-demo.png".to_string(),
        headers: Some(metadata_headers),
    };

    let source4 = client.source_from_file("gcs_input.png").await?;
    match source4.store(StoreOptions::GCS(gcs_metadata_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored with rich metadata!");
        }
        Err(e) => {
            println!("   ❌ GCS metadata storage error: {}", e);
        }
    }

    // Example 5: Batch storage simulation
    println!("\n📦 Example 5: Batch storage simulation");
    let batch_files = [
        "batch-bucket/batch-001/image-01.png",
        "batch-bucket/batch-001/image-02.png",
        "batch-bucket/batch-001/image-03.png",
    ];

    for (index, path) in batch_files.iter().enumerate() {
        let batch_headers = json!({
            "X-Goog-Meta-Batch-Id": "batch-001",
            "X-Goog-Meta-Item-Index": index,
            "Cache-Control": "public, max-age=3600"
        });

        let gcs_batch_options = GCSOptions {
            service: "gcs".to_string(),
            gcp_access_token: "DEMO_GCP_ACCESS_TOKEN".to_string(),
            path: path.to_string(),
            headers: Some(batch_headers),
        };

        let source_batch = client.source_from_file("gcs_input.png").await?;
        match source_batch
            .store(StoreOptions::GCS(gcs_batch_options))
            .await
        {
            Ok(_) => {
                println!("   ✅ Batch item {} stored: {}", index + 1, path);
            }
            Err(e) => {
                println!("   ❌ Batch item {} error: {}", index + 1, e);
            }
        }
    }

    // Example 6: Environment variable and authentication info
    println!("\n🔐 Example 6: Authentication and configuration");
    println!("   To use real GCS credentials, you have several options:");
    println!();
    println!("   Option 1 - Access Token:");
    println!("   export GCP_ACCESS_TOKEN='your-oauth-access-token'");
    println!();
    println!("   Option 2 - Service Account (recommended):");
    println!("   export GOOGLE_APPLICATION_CREDENTIALS='/path/to/service-account.json'");
    println!("   # Then use: gcloud auth application-default print-access-token");
    println!();
    println!("   Option 3 - gcloud CLI:");
    println!("   gcloud auth login");
    println!("   gcloud auth print-access-token");
    println!();
    println!("   Current configuration:");
    println!(
        "   GCP_ACCESS_TOKEN: {}",
        if std::env::var("GCP_ACCESS_TOKEN").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set (using demo)"
        }
    );
    println!(
        "   GOOGLE_APPLICATION_CREDENTIALS: {}",
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set"
        }
    );

    // Example 7: Demonstrating different content types
    println!("\n🎨 Example 7: Different image formats to GCS");
    use tinify::{ConvertOptions, ImageFormat};

    let formats = vec![
        (ImageFormat::Png, "png", "image/png"),
        (ImageFormat::Jpeg, "jpg", "image/jpeg"),
        (ImageFormat::WebP, "webp", "image/webp"),
    ];

    for (format, ext, content_type) in formats {
        let convert_options = ConvertOptions {
            format,
            background: Some("#FFFFFF".to_string()),
        };

        let source_convert = client.source_from_file("gcs_input.png").await?;
        match source_convert.convert(convert_options).await {
            Ok(converted_result) => {
                // Store the converted image to GCS
                let format_headers = json!({
                    "Content-Type": content_type,
                    "X-Goog-Meta-Original-Format": "PNG",
                    "X-Goog-Meta-Converted-Format": ext.to_uppercase()
                });

                let source_bytes = converted_result.into();
                let source_converted = client.source_from_buffer(source_bytes).await?;

                let gcs_format_options = GCSOptions {
                    service: "gcs".to_string(),
                    gcp_access_token: "DEMO_GCP_ACCESS_TOKEN".to_string(),
                    path: format!("format-demo-bucket/converted/image.{}", ext),
                    headers: Some(format_headers),
                };

                match source_converted
                    .store(StoreOptions::GCS(gcs_format_options))
                    .await
                {
                    Ok(_) => {
                        println!("   ✅ {} format stored successfully", ext.to_uppercase());
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

    println!("\n🎉 Google Cloud Storage examples completed!");
    println!("\nℹ️  GCS Storage Notes:");
    println!("   • Authentication: OAuth2 access token required");
    println!("   • Path format: bucket-name/path/to/file.extension");
    println!("   • Custom headers: X-Goog-Meta-* for metadata, Cache-Control, etc.");
    println!("   • Access tokens expire (typically 1 hour) - refresh as needed");
    println!("   • Storage does not count as additional compression");
    println!("   • Supports all standard HTTP headers and GCS-specific metadata");

    // Clean up test files
    cleanup_files(&["gcs_input.png"]).await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_simple_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_simple_png_data() -> Vec<u8> {
    // Simple PNG for GCS storage testing
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, // 24x24 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0xE0, 0x77, 0x3D, // RGBA format
        0xF8, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x64, 0x60, 0x60, 0xF8, // Minimal image data
        0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x30,
        0x31, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
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
