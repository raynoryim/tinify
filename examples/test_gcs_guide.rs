use serde_json::json;
use std::error::Error;
use tinify::{GCSOptions, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("☁️  Tinify: GCS Testing Guide");
    println!("===============================");
    println!();

    println!("ℹ️  Google Cloud Storage Testing Options:");
    println!("   Option 1: GCS Free Tier ($300 credits + always-free tier)");
    println!("            Sign up at: https://cloud.google.com/free");
    println!("   Option 2: GCS Emulator (local testing)");
    println!("            docker run -p 4443:4443 fsouza/fake-gcs-server -scheme http");
    println!("   Option 3: BigQuery Sandbox (limited GCS features)");
    println!("            No credit card required: https://cloud.google.com/bigquery/docs/sandbox");
    println!();

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Tinify client initialized");

    // Create test image
    create_test_image("gcs_test_input.png").await?;

    let source = client.source_from_file("gcs_test_input.png").await?;
    println!("✅ Source image loaded");

    // Test 1: GCS with Demo Token (will fail but show expected behavior)
    println!("\n🧪 Test 1: GCS with Demo Token");

    let demo_gcs_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: "demo-token-for-testing".to_string(),
        path: "test-bucket-gcs/compressed-image.png".to_string(),
        headers: None,
    };

    match source.store(StoreOptions::GCS(demo_gcs_options)).await {
        Ok(result) => {
            println!("   ✅ Image stored to GCS successfully! (unexpected)");
            if let Some(compression_count) = result.compression_count() {
                println!("   📈 Compression count: {}", compression_count);
            }
        }
        Err(e) => {
            println!("   ❌ GCS storage failed (expected with demo token): {}", e);
            println!("   💡 This is expected behavior with demo credentials");
        }
    }

    // Test 2: GCS with Environment Token
    println!("\n🔑 Test 2: GCS with Environment Token");

    match std::env::var("GCP_ACCESS_TOKEN") {
        Ok(token) => {
            println!("   ✅ Found GCP_ACCESS_TOKEN environment variable");

            let real_gcs_options = GCSOptions {
                service: "gcs".to_string(),
                gcp_access_token: token,
                path: "test-bucket-gcs/compressed-with-real-token.png".to_string(),
                headers: Some(json!({
                    "Cache-Control": "public, max-age=3600",
                    "X-Goog-Meta-Source": "tinify-test",
                    "X-Goog-Meta-Timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            };

            let source2 = client.source_from_file("gcs_test_input.png").await?;
            match source2.store(StoreOptions::GCS(real_gcs_options)).await {
                Ok(_) => {
                    println!("   ✅ Image stored to GCS with real token!");
                }
                Err(e) => {
                    println!("   ❌ GCS storage failed: {}", e);
                    println!("   💡 Check your GCP access token and bucket permissions");
                }
            }
        }
        Err(_) => {
            println!("   ℹ️  No GCP_ACCESS_TOKEN found in environment");
            println!("   💡 To test with real GCS, set GCP_ACCESS_TOKEN:");
            println!("      export GCP_ACCESS_TOKEN=$(gcloud auth application-default print-access-token)");
        }
    }

    // Test 3: How to Get GCP Access Token
    println!("\n🔧 Test 3: How to Obtain GCP Access Token");

    println!("   Methods to get GCP access token:");
    println!("   ┌─────────────────────────────────────────────────────────────┐");
    println!("   │ Method 1: gcloud CLI                                       │");
    println!("   │   gcloud auth login                                        │");
    println!("   │   gcloud auth application-default print-access-token       │");
    println!("   │                                                             │");
    println!("   │ Method 2: Service Account                                   │");
    println!("   │   export GOOGLE_APPLICATION_CREDENTIALS=/path/to/key.json  │");
    println!("   │   gcloud auth application-default print-access-token       │");
    println!("   │                                                             │");
    println!("   │ Method 3: Compute Engine (auto)                           │");
    println!("   │   curl -H \"Metadata-Flavor: Google\" \\                      │");
    println!("   │   http://metadata.google.internal/computeMetadata/v1/...   │");
    println!("   └─────────────────────────────────────────────────────────────┘");

    // Test 4: GCS Configuration Examples
    println!("\n📋 Test 4: GCS Configuration Examples");

    let gcs_configs = vec![
        (
            "Basic Upload",
            GCSOptions {
                service: "gcs".to_string(),
                gcp_access_token: "your-token".to_string(),
                path: "my-bucket/image.png".to_string(),
                headers: None,
            },
        ),
        (
            "With Cache Headers",
            GCSOptions {
                service: "gcs".to_string(),
                gcp_access_token: "your-token".to_string(),
                path: "my-bucket/cached/image.png".to_string(),
                headers: Some(json!({
                    "Cache-Control": "public, max-age=31536000",
                    "Content-Type": "image/png"
                })),
            },
        ),
        (
            "With Metadata",
            GCSOptions {
                service: "gcs".to_string(),
                gcp_access_token: "your-token".to_string(),
                path: "my-bucket/metadata/image.png".to_string(),
                headers: Some(json!({
                    "X-Goog-Meta-Project": "my-project",
                    "X-Goog-Meta-Environment": "production",
                    "X-Goog-Meta-Compressed-By": "tinify"
                })),
            },
        ),
    ];

    for (name, config) in gcs_configs {
        println!("   📁 {}: {}", name, config.path);
        if let Some(ref headers) = config.headers {
            println!("      Headers: {}", serde_json::to_string_pretty(headers)?);
        }
        println!();
    }

    // Test 5: Cost Calculation for GCS
    println!("\n💰 Test 5: GCS Cost Analysis");

    println!("   GCS Pricing (as of 2024):");
    println!("   ├── Standard Storage: $0.020 per GB/month");
    println!("   ├── Nearline Storage: $0.010 per GB/month");
    println!("   ├── Coldline Storage: $0.004 per GB/month");
    println!("   └── Archive Storage: $0.0012 per GB/month");
    println!();
    println!("   Free Tier Benefits:");
    println!("   ├── 5 GB storage per month");
    println!("   ├── 1 GB network egress per month");
    println!("   └── Class A operations: 5,000 per month");
    println!();
    println!("   💡 Tinify + GCS Direct Upload Benefits:");
    println!("      • No client bandwidth usage");
    println!("      • No intermediate storage costs");
    println!("      • Reduced latency");
    println!("      • Simplified architecture");

    // Test 6: Format Conversion + GCS
    println!("\n🎨 Test 6: Format Conversion + GCS (Simulation)");

    use tinify::{ConvertOptions, ImageFormat};

    let formats = vec![
        (ImageFormat::Jpeg, "jpg", "image/jpeg"),
        (ImageFormat::WebP, "webp", "image/webp"),
    ];

    for (format, ext, content_type) in formats {
        println!("   🔄 Converting to {} format...", ext.to_uppercase());

        let convert_options = ConvertOptions {
            format,
            background: Some("#FFFFFF".to_string()),
        };

        let source_convert = client.source_from_file("gcs_test_input.png").await?;
        match source_convert.convert(convert_options).await {
            Ok(converted_result) => {
                println!("      ✅ Conversion successful");

                // Simulate GCS upload (will fail with demo token but shows structure)
                let converted_bytes = converted_result.into();
                let source_converted = client.source_from_buffer(converted_bytes).await?;

                let gcs_format_options = GCSOptions {
                    service: "gcs".to_string(),
                    gcp_access_token: "demo-token".to_string(),
                    path: format!("test-bucket-gcs/converted.{}", ext),
                    headers: Some(json!({
                        "Content-Type": content_type,
                        "X-Goog-Meta-Original-Format": "PNG",
                        "X-Goog-Meta-Conversion": format!("PNG-to-{}", ext.to_uppercase())
                    })),
                };

                match source_converted
                    .store(StoreOptions::GCS(gcs_format_options))
                    .await
                {
                    Ok(_) => {
                        println!("      ✅ {} uploaded to GCS", ext.to_uppercase());
                    }
                    Err(_) => {
                        println!(
                            "      ❌ {} upload failed (expected with demo token)",
                            ext.to_uppercase()
                        );
                    }
                }
            }
            Err(e) => {
                println!("      ❌ Conversion to {} failed: {}", ext, e);
            }
        }
    }

    // Test 7: Authentication Status
    println!("\n🔐 Test 7: Authentication Status Check");

    println!("   Environment Variables:");
    println!(
        "   ├── TINIFY_API_KEY: {}",
        if std::env::var("TINIFY_API_KEY").is_ok() {
            "✅ Set"
        } else {
            "❌ Using provided key"
        }
    );
    println!(
        "   ├── GCP_ACCESS_TOKEN: {}",
        if std::env::var("GCP_ACCESS_TOKEN").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set"
        }
    );
    println!(
        "   └── GOOGLE_APPLICATION_CREDENTIALS: {}",
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set"
        }
    );

    println!("\n   Next Steps for Real GCS Testing:");
    println!("   1️⃣  Sign up for GCS free tier: https://cloud.google.com/free");
    println!("   2️⃣  Install gcloud CLI: https://cloud.google.com/sdk/docs/install");
    println!("   3️⃣  Authenticate: gcloud auth login");
    println!("   4️⃣  Get token: gcloud auth application-default print-access-token");
    println!("   5️⃣  Set env: export GCP_ACCESS_TOKEN=<your-token>");
    println!("   6️⃣  Create bucket: gsutil mb gs://your-test-bucket");
    println!("   7️⃣  Re-run this example");

    println!("\n🎉 GCS Testing Guide Completed!");
    println!("   This example demonstrates GCS integration patterns");
    println!("   and shows how to configure real GCS authentication.");

    // Cleanup
    cleanup_files(&["gcs_test_input.png"]).await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_simple_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_simple_png_data() -> Vec<u8> {
    // Simple PNG for GCS testing
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x32, // 50x50 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1E, 0x3F, 0x88, // RGBA format
        0xB1, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x6C, 0x60, 0x60, 0xF8, 0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01,
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
