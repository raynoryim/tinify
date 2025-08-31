use std::error::Error;
use tinify::{S3Options, StoreOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("💾 Tinify: S3 Compatible Storage Example");
    println!("==========================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create test image
    create_test_image("s3_compatible_input.png").await?;

    let source = client.source_from_file("s3_compatible_input.png").await?;
    println!("✅ Source image loaded");

    // Example 1: Digital Ocean Spaces (S3-compatible)
    println!("\n🌊 Example 1: Digital Ocean Spaces");
    let do_spaces_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("DO_SPACES_KEY")
            .unwrap_or_else(|_| "DEMO_DO_SPACES_KEY".to_string()),
        aws_secret_access_key: std::env::var("DO_SPACES_SECRET")
            .unwrap_or_else(|_| "DEMO_DO_SPACES_SECRET".to_string()),
        region: "nyc3".to_string(), // DigitalOcean region
        path: "my-space/images/compressed-image.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    match source.store(StoreOptions::S3(do_spaces_options)).await {
        Ok(result) => {
            println!("   ✅ Image stored to DigitalOcean Spaces!");
            if let Some(count) = result.compression_count() {
                println!("   📈 Compression count: {}", count);
            }
        }
        Err(e) => {
            println!("   ❌ DigitalOcean Spaces error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 2: Backblaze B2 (S3-compatible)
    println!("\n💽 Example 2: Backblaze B2 Cloud Storage");
    let b2_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("B2_APPLICATION_KEY_ID")
            .unwrap_or_else(|_| "DEMO_B2_KEY_ID".to_string()),
        aws_secret_access_key: std::env::var("B2_APPLICATION_KEY")
            .unwrap_or_else(|_| "DEMO_B2_APPLICATION_KEY".to_string()),
        region: "us-west-002".to_string(), // Backblaze region
        path: "my-bucket/compressed/image.png".to_string(),
        headers: None,
        acl: None, // Backblaze handles ACL differently
    };

    let source2 = client.source_from_file("s3_compatible_input.png").await?;
    match source2.store(StoreOptions::S3(b2_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to Backblaze B2!");
        }
        Err(e) => {
            println!("   ❌ Backblaze B2 error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 3: Wasabi Hot Cloud Storage
    println!("\n🌶️  Example 3: Wasabi Hot Cloud Storage");
    let wasabi_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("WASABI_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_WASABI_ACCESS_KEY".to_string()),
        aws_secret_access_key: std::env::var("WASABI_SECRET_KEY")
            .unwrap_or_else(|_| "DEMO_WASABI_SECRET_KEY".to_string()),
        region: "us-east-1".to_string(), // Wasabi region
        path: "my-bucket/optimized/image.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    let source3 = client.source_from_file("s3_compatible_input.png").await?;
    match source3.store(StoreOptions::S3(wasabi_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to Wasabi!");
        }
        Err(e) => {
            println!("   ❌ Wasabi storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 4: G-Core Labs Cloud Storage
    println!("\n⚡ Example 4: G-Core Labs Cloud Storage");
    let gcore_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("GCORE_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_GCORE_ACCESS_KEY".to_string()),
        aws_secret_access_key: std::env::var("GCORE_SECRET_KEY")
            .unwrap_or_else(|_| "DEMO_GCORE_SECRET_KEY".to_string()),
        region: "ams".to_string(), // G-Core region
        path: "my-storage/images/compressed.png".to_string(),
        headers: None,
        acl: None,
    };

    let source4 = client.source_from_file("s3_compatible_input.png").await?;
    match source4.store(StoreOptions::S3(gcore_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to G-Core Labs!");
        }
        Err(e) => {
            println!("   ❌ G-Core Labs error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 5: MinIO (Self-hosted S3)
    println!("\n🏠 Example 5: MinIO (Self-hosted S3)");
    let minio_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: std::env::var("MINIO_ACCESS_KEY")
            .unwrap_or_else(|_| "DEMO_MINIO_ACCESS_KEY".to_string()),
        aws_secret_access_key: std::env::var("MINIO_SECRET_KEY")
            .unwrap_or_else(|_| "DEMO_MINIO_SECRET_KEY".to_string()),
        region: "us-east-1".to_string(), // MinIO default region
        path: "my-bucket/uploads/processed.png".to_string(),
        headers: None,
        acl: None,
    };

    let source5 = client.source_from_file("s3_compatible_input.png").await?;
    match source5.store(StoreOptions::S3(minio_options)).await {
        Ok(_) => {
            println!("   ✅ Image stored to MinIO!");
        }
        Err(e) => {
            println!("   ❌ MinIO storage error: {}", e);
            println!("   ℹ️  This is expected with demo credentials");
        }
    }

    // Example 6: Environment variable configuration patterns
    println!("\n🔧 Example 6: Configuration patterns for S3-compatible services");

    let services = vec![
        (
            "DigitalOcean Spaces",
            vec![
                "DO_SPACES_KEY=your-spaces-access-key",
                "DO_SPACES_SECRET=your-spaces-secret-key",
                "# Region: nyc3, fra1, sfo3, etc.",
            ],
        ),
        (
            "Backblaze B2",
            vec![
                "B2_APPLICATION_KEY_ID=your-key-id",
                "B2_APPLICATION_KEY=your-application-key",
                "# Region: us-west-002, eu-central-003, etc.",
            ],
        ),
        (
            "Wasabi",
            vec![
                "WASABI_ACCESS_KEY=your-access-key",
                "WASABI_SECRET_KEY=your-secret-key",
                "# Region: us-east-1, eu-central-1, ap-northeast-1, etc.",
            ],
        ),
        (
            "G-Core Labs",
            vec![
                "GCORE_ACCESS_KEY=your-access-key",
                "GCORE_SECRET_KEY=your-secret-key",
                "# Region: ams, fra, sin, etc.",
            ],
        ),
        (
            "MinIO",
            vec![
                "MINIO_ACCESS_KEY=your-minio-access-key",
                "MINIO_SECRET_KEY=your-minio-secret-key",
                "# Usually self-hosted with custom endpoint",
            ],
        ),
    ];

    for (service, env_vars) in services {
        println!("   📦 {}:", service);
        for var in env_vars {
            println!("      {}", var);
        }
        println!();
    }

    // Example 7: Cost and performance comparison
    println!("\n💰 Example 7: S3-Compatible Services Comparison");

    let comparison = vec![
        (
            "AWS S3",
            "Standard",
            "High reliability, global",
            "$0.023/GB/month",
        ),
        (
            "DigitalOcean Spaces",
            "Simple",
            "Easy to use, good pricing",
            "$0.02/GB/month",
        ),
        (
            "Backblaze B2",
            "Low-cost",
            "Very affordable, pay-per-use",
            "$0.005/GB/month",
        ),
        (
            "Wasabi",
            "Hot storage",
            "Unlimited egress, fast",
            "$0.0059/GB/month",
        ),
        (
            "G-Core Labs",
            "Global CDN",
            "CDN integrated, fast delivery",
            "$0.03/GB/month",
        ),
        (
            "MinIO",
            "Self-hosted",
            "Full control, on-premises",
            "Infrastructure costs only",
        ),
    ];

    println!("   📊 Service Comparison:");
    println!("   ┌─────────────────┬─────────────┬──────────────────────┬──────────────────┐");
    println!("   │ Service         │ Type        │ Key Feature          │ Pricing Est.     │");
    println!("   ├─────────────────┼─────────────┼──────────────────────┼──────────────────┤");
    for (service, stype, feature, pricing) in comparison {
        println!(
            "   │ {:<15} │ {:<11} │ {:<20} │ {:<16} │",
            service, stype, feature, pricing
        );
    }
    println!("   └─────────────────┴─────────────┴──────────────────────┴──────────────────┘");

    println!("\n🎉 S3-compatible storage examples completed!");
    println!("\nℹ️  S3-Compatible Storage Notes:");
    println!("   • All use the same S3Options structure");
    println!("   • Different endpoint URLs (handled by Tinify API)");
    println!("   • Region names vary between providers");
    println!("   • Some providers have different ACL systems");
    println!("   • Pricing and features differ significantly");
    println!("   • All provide cost savings compared to downloading/uploading manually");

    // Clean up test files
    cleanup_files(&["s3_compatible_input.png"]).await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_simple_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_simple_png_data() -> Vec<u8> {
    // Simple PNG for S3-compatible storage testing
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, // 32x32 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x73, 0x7A, 0x7A, // RGBA format
        0xF4, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x68, 0x60, 0x60, 0xF8, 0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01,
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
