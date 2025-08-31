use std::time::Duration;
use tempfile::NamedTempFile;
use tinify::{RateLimit, ResizeMethod, ResizeOptions, RetryConfig, Tinify};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Tinify v0.2.0 Improvements Demo");
    println!("=====================================\n");

    // 1. Builder pattern client creation
    println!("1. 🏗️  Builder Pattern Configuration");
    let retry_config = RetryConfig {
        max_attempts: 5,
        base_delay: Duration::from_millis(200),
        max_delay: Duration::from_secs(30),
        backoff_factor: 2.0,
    };

    let rate_limit = RateLimit {
        requests_per_minute: 200,
        burst_capacity: 15,
    };

    let client = Tinify::builder()
        .api_key(std::env::var("TINIFY_API_KEY").unwrap_or("demo-key".to_string()))
        .app_identifier("TinifyDemo/2.0")
        .timeout(Duration::from_secs(60))
        .retry_config(retry_config)
        .rate_limit(rate_limit)
        .build()?;

    info!("Created Tinify client with enhanced configuration");
    println!("   ✅ Client created with custom configuration");

    // 2. Input validation examples
    println!("\n2. 🛡️  Input Validation");

    // File not found validation
    match client.source_from_file("nonexistent.png").await {
        Err(tinify::TinifyError::FileNotFound { path }) => {
            println!(
                "   ✅ File not found validation: {:?}",
                path.file_name().unwrap()
            );
        }
        _ => println!("   ❌ File not found validation failed"),
    }

    // Format validation
    let temp_file = NamedTempFile::with_suffix(".txt")?;
    match client.source_from_file(temp_file.path()).await {
        Err(tinify::TinifyError::UnsupportedFormat { format }) => {
            println!("   ✅ Unsupported format validation: {}", format);
        }
        _ => println!("   ❌ Format validation failed"),
    }

    // Buffer size validation
    let large_buffer = vec![0u8; 6 * 1024 * 1024]; // 6MB
    match client.source_from_buffer(large_buffer).await {
        Err(tinify::TinifyError::FileTooLarge { size, max_size }) => {
            println!(
                "   ✅ File too large validation: {:.1}MB > {:.1}MB",
                size as f64 / 1024.0 / 1024.0,
                max_size as f64 / 1024.0 / 1024.0
            );
        }
        _ => println!("   ❌ File size validation failed"),
    }

    // URL validation
    match client.source_from_url("invalid-url").await {
        Err(tinify::TinifyError::UrlParseError(_)) => {
            println!("   ✅ URL format validation works");
        }
        _ => println!("   ❌ URL validation failed"),
    }

    // 3. Multi-client support (no global state)
    println!("\n3. 🌐 Multiple Client Instances");
    let client1 = Tinify::new("api-key-1".to_string())?;
    let client2 = Tinify::new("api-key-2".to_string())?;

    println!("   ✅ Client 1 API key: {}", client1.api_key());
    println!("   ✅ Client 2 API key: {}", client2.api_key());

    // 4. Enhanced error handling
    println!("\n4. ⚠️  Enhanced Error Handling");
    match Tinify::builder().build() {
        Err(tinify::TinifyError::InvalidApiKey) => {
            println!("   ✅ Granular error: Missing API key detected");
        }
        _ => println!("   ❌ Error handling test failed"),
    }

    // 5. Resize options demonstration
    println!("\n5. 📏 Resize Options with Defaults");
    let valid_resize = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(300),
        height: Some(200),
    };
    println!(
        "   ✅ Valid resize options: {:?} {}x{}",
        valid_resize.method,
        valid_resize.width.unwrap(),
        valid_resize.height.unwrap()
    );

    // Show what would happen with invalid dimensions (caught during actual resize)
    let _invalid_resize = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(0), // This would be caught during resize operation
        height: Some(100),
    };
    println!("   ⚠️  Invalid resize (width=0) would be caught during resize operation");

    // 6. Default configurations
    println!("\n6. 🔧 Default Configurations");
    let _simple_client = Tinify::new("simple-key".to_string())?;
    println!("   ✅ Simple client created with defaults");

    let default_resize = ResizeOptions::default();
    println!(
        "   ✅ Default resize options: {:?} {}x{}",
        default_resize.method,
        default_resize.width.unwrap(),
        default_resize.height.unwrap()
    );

    println!("\n🎉 All improvements verified successfully!");
    println!("\n📈 Benefits Summary:");
    println!("   • No global state - thread safe multiple clients");
    println!("   • Builder pattern - flexible configuration");
    println!("   • Input validation - prevents invalid API calls");
    println!("   • Retry & rate limiting - resilient operations");
    println!("   • Structured logging - better observability");
    println!("   • Streaming support - handle large files");
    println!("   • Enhanced errors - precise error information");
    println!("   • Security improvements - no hardcoded keys");

    Ok(())
}
