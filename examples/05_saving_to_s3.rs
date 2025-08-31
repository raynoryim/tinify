use serde_json::json;
use std::error::Error;
use tinify_rs::{S3Options, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("📦 Tinify-rs: Saving to Amazon S3 Example");
    println!("========================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create test image
    create_test_image("s3_input.png").await?;

    let source = client.source_from_file("s3_input.png").await?;
    println!("✅ Source image loaded");

    // Example 1: Basic S3 storage
    println!("\n☁️  Example 1: Basic S3 storage");
    let s3_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "DEMO_ACCESS_KEY_ID".to_string()),
        aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_SECRET_ACCESS_KEY".to_string()),
        region: "us-east-1".to_string(),
        path: "my-bucket/images/compressed-image.png".to_string(),
        headers: None,
        acl: None,
    };

    match source.store(StoreOptions::S3(s3_options)).await {
        Ok(result) => {
            println!("   ✅ Image stored to S3 successfully!");
            if let Some(compression_count) = result.compression_count() {
                println!("   📈 Compression count: {}", compression_count);
            }
        }
        Err(e) => {
            println!("   ❌ S3 storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 2: S3 storage with public-read ACL
    println!("\n🌐 Example 2: S3 storage with public-read ACL");
    let s3_public_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "DEMO_ACCESS_KEY_ID".to_string()),
        aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_SECRET_ACCESS_KEY".to_string()),
        region: "us-west-2".to_string(),
        path: "my-public-bucket/images/public-image.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    let source2 = client.source_from_file("s3_input.png").await?;
    match source2.store(StoreOptions::S3(s3_public_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to S3 with public-read ACL!");
        }
        Err(e) => {
            println!("   ❌ S3 public storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 3: S3 storage with custom headers
    println!("\n📋 Example 3: S3 storage with custom headers");
    let custom_headers = json!({
        "Cache-Control": "public, max-age=31536000",
        "Expires": "Sun, 01 Jan 2025 00:00:00 GMT",
        "Content-Disposition": "inline; filename=\"optimized-image.png\""
    });

    let s3_headers_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "DEMO_ACCESS_KEY_ID".to_string()),
        aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_SECRET_ACCESS_KEY".to_string()),
        region: "eu-west-1".to_string(),
        path: "my-cdn-bucket/assets/cached-image.png".to_string(),
        headers: Some(custom_headers),
        acl: Some("public-read".to_string()),
    };

    let source3 = client.source_from_file("s3_input.png").await?;
    match source3.store(StoreOptions::S3(s3_headers_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to S3 with custom headers!");
        }
        Err(e) => {
            println!("   ❌ S3 headers storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 4: S3 storage with different regions
    println!("\n🌍 Example 4: S3 storage in different regions");
    let regions = vec![
        ("us-east-1", "my-us-east-bucket"),
        ("us-west-2", "my-us-west-bucket"),
        ("eu-west-1", "my-eu-bucket"),
        ("ap-southeast-1", "my-asia-bucket"),
    ];

    for (region, bucket) in regions {
        let s3_region_options = S3Options {
            service: "s3".to_string(),
            aws_access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
                .unwrap_or_else(|_| "DEMO_ACCESS_KEY_ID".to_string()),
            aws_secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
                .unwrap_or_else(|_| "DEMO_SECRET_ACCESS_KEY".to_string()),
            region: region.to_string(),
            path: format!("{}/images/region-test.png", bucket),
            headers: None,
            acl: None,
        };

        let source_region = client.source_from_file("s3_input.png").await?;
        match source_region
            .store(StoreOptions::S3(s3_region_options))
            .await
        {
            Ok(_) => {
                println!("   ✅ Stored to {} region successfully", region);
            }
            Err(e) => {
                println!("   ❌ {} region storage error: {}", region, e);
            }
        }
    }

    // Example 5: Demonstrating S3 path variations
    println!("\n📁 Example 5: Different S3 path structures");
    let path_examples = vec![
        "my-bucket/simple-name.png",
        "my-bucket/folder/subfolder/nested.png",
        "my-bucket/images/2024/01/dated.png",
        "my-bucket/user-uploads/123456/profile.png",
        "my-bucket/cdn/thumbnails/thumb_100x100.png",
    ];

    for path in path_examples {
        let s3_path_options = S3Options {
            service: "s3".to_string(),
            aws_access_key_id: "DEMO_ACCESS_KEY_ID".to_string(),
            aws_secret_access_key: "DEMO_SECRET_ACCESS_KEY".to_string(),
            region: "us-east-1".to_string(),
            path: path.to_string(),
            headers: None,
            acl: None,
        };

        let source_path = client.source_from_file("s3_input.png").await?;
        match source_path.store(StoreOptions::S3(s3_path_options)).await {
            Ok(_) => {
                println!("   ✅ Path structure '{}' accepted", path);
            }
            Err(e) => {
                println!("   ❌ Path '{}' error: {}", path, e);
            }
        }
    }

    // Example 6: Environment variable configuration demo
    println!("\n🔧 Example 6: Environment variable configuration");
    println!("   To use real S3 credentials, set these environment variables:");
    println!("   export AWS_ACCESS_KEY_ID='your-access-key-id'");
    println!("   export AWS_SECRET_ACCESS_KEY='your-secret-access-key'");
    println!("   export TINIFY_API_KEY='your-tinify-api-key'");
    println!();
    println!("   Current configuration:");
    println!(
        "   AWS_ACCESS_KEY_ID: {}",
        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set (using demo)"
        }
    );
    println!(
        "   AWS_SECRET_ACCESS_KEY: {}",
        if std::env::var("AWS_SECRET_ACCESS_KEY").is_ok() {
            "✅ Set"
        } else {
            "❌ Not set (using demo)"
        }
    );

    println!("\n🎉 S3 storage examples completed!");
    println!("\nℹ️  S3 Storage Notes:");
    println!("   • Supported ACLs: private, public-read, public-read-write, authenticated-read");
    println!("   • Custom headers: Cache-Control, Expires, Content-Disposition, etc.");
    println!("   • Regions: Any valid AWS region (us-east-1, eu-west-1, ap-southeast-1, etc.)");
    println!("   • Path format: bucket-name/path/to/file.extension");
    println!("   • Storage does not count as additional compression");

    // Clean up test files
    cleanup_files(&["s3_input.png"]).await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_simple_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_simple_png_data() -> Vec<u8> {
    // Simple PNG for S3 storage testing
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x10, // 16x16 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0xF3, 0xFF, // RGBA format
        0x61, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x62, 0x60, 0x60, 0xF8, // Minimal image data
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
