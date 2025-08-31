# Tinify

[![Crates.io](https://img.shields.io/crates/v/tinify.svg)](https://crates.io/crates/tinify)
[![Documentation](https://docs.rs/tinify/badge.svg)](https://docs.rs/tinify)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/raynoryim/tinify/workflows/CI/badge.svg)](https://github.com/raynoryim/tinify/actions)

**English** | [中文](README_CN.md)

A high-performance Rust library for image compression and optimization, built on the [TinyPNG API](https://tinypng.com/developers). Provides async support, intelligent retry mechanisms, rate limiting, and cloud storage integration.

## ✨ Features

- 🖼️ **Smart Compression**: Lossless quality PNG/JPEG/WebP/AVIF image compression
- 📏 **Image Resizing**: Multiple resize methods (scale/fit/cover/thumb)
- 🔄 **Format Conversion**: Convert between popular image formats
- 📊 **Metadata Preservation**: Optionally preserve copyright, creation time, location data
- ☁️ **Cloud Storage**: Direct upload to AWS S3, Google Cloud Storage
- 🚀 **High-Performance Async**: Built on tokio for concurrent processing
- 🛡️ **Type Safety**: Full Rust type system and comprehensive error handling
- ⚡ **Smart Retry**: Built-in exponential backoff retry logic and rate limiting
- 📦 **Zero Config**: Works out of the box with minimal setup

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tinify = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

### Basic Usage

```rust
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Tinify::new("your-api-key".to_string())?;

    // Compress image
    let source = client.source_from_file("input.png").await?;
    source.to_file("output.png").await?;

    println!("Image compression completed!");
    Ok(())
}
```

### Advanced Configuration

```rust
use tinify::Tinify;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use builder pattern for advanced configuration
    let client = Tinify::builder()
        .api_key("your-api-key")
        .app_identifier("MyApp/1.0")
        .timeout(Duration::from_secs(30))
        .max_retry_attempts(3)
        .requests_per_minute(100)
        .build()?;

    let source = client.source_from_file("input.png").await?;
    source.to_file("output.png").await?;

    Ok(())
}
```

## 📖 Detailed Examples

### Image Resizing

```rust
use tinify::{Tinify, ResizeOptions, ResizeMethod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // Configure resize options
    let resize_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(300),
        height: Some(200),
    };

    // Resize image
    let mut result = source.resize(resize_options).await?;
    result.to_file("resized.png").await?;

    // Get image information
    if let Some(width) = result.image_width() {
        println!("Resized width: {} pixels", width);
    }

    Ok(())
}
```

### Format Conversion

```rust
use tinify::{Tinify, ConvertOptions, ImageFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // Convert to WebP format
    let convert_options = ConvertOptions {
        format: ImageFormat::WebP,
        background: Some("#FFFFFF".to_string()),
    };

    let mut result = source.convert(convert_options).await?;
    result.to_file("output.webp").await?;

    Ok(())
}
```

### Metadata Preservation

```rust
use tinify::{Tinify, PreserveOptions, PreserveMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.jpg").await?;

    // Preserve copyright and creation time
    let preserve_options = PreserveOptions {
        preserve: vec![
            PreserveMetadata::Copyright,
            PreserveMetadata::Creation,
        ],
    };

    let mut result = source.preserve(preserve_options).await?;
    result.to_file("preserved.jpg").await?;

    Ok(())
}
```

### AWS S3 Cloud Storage

```rust
use tinify::{Tinify, StoreOptions, S3Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // Configure S3 storage options
    let s3_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: "your-access-key".to_string(),
        aws_secret_access_key: "your-secret-key".to_string(),
        region: "us-east-1".to_string(),
        path: "my-bucket/images/compressed.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    // Store directly to S3
    let result = source.store(StoreOptions::S3(s3_options)).await?;

    if let Some(count) = result.compression_count() {
        println!("API usage count: {}", count);
    }

    Ok(())
}
```

### Google Cloud Storage

```rust
use tinify::{Tinify, StoreOptions, GCSOptions};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // Configure GCS storage options
    let gcs_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: "your-access-token".to_string(),
        path: "my-bucket/images/compressed.png".to_string(),
        headers: Some(json!({
            "Cache-Control": "public, max-age=31536000",
            "X-Goog-Meta-Source": "tinify-rs"
        })),
    };

    // Store directly to GCS
    let result = source.store(StoreOptions::GCS(gcs_options)).await?;

    Ok(())
}
```

### URL-based Processing

```rust
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;

    // Load image from URL
    let source = client.source_from_url("https://example.com/image.jpg").await?;
    source.to_file("compressed.jpg").await?;

    Ok(())
}
```

### Buffer-based Processing

```rust
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;

    // Create source from in-memory bytes
    let image_data = std::fs::read("input.png")?;
    let source = client.source_from_buffer(image_data).await?;

    // Get compressed bytes
    let compressed_data = source.to_buffer().await?;
    std::fs::write("output.png", compressed_data)?;

    Ok(())
}
```

## 🔧 API Reference

### Resize Methods

| Method | Description | Use Case |
|--------|-------------|----------|
| `Scale` | Proportional scaling | Precise width or height control |
| `Fit` | Fit within dimensions (preserve aspect ratio) | Create largest image within bounds |
| `Cover` | Cover dimensions (may crop) | Fill exact dimensions, preserve ratio |
| `Thumb` | Smart thumbnail | Auto-detect important regions |

### Supported Image Formats

| Format | Input Support | Output Support | Description |
|--------|---------------|----------------|-------------|
| PNG | ✅ | ✅ | Lossless compression, transparency support |
| JPEG | ✅ | ✅ | Lossy compression, ideal for photos |
| WebP | ✅ | ✅ | Modern format, smaller file sizes |
| AVIF | ❌ | ✅ | Next-gen format, best compression |

### Cloud Storage Support

| Service | Support Status | Notes |
|---------|----------------|--------|
| AWS S3 | ✅ | Full support with custom headers and ACL |
| Google Cloud Storage | ✅ | Full support with metadata |
| S3-Compatible Services | ✅ | MinIO, DigitalOcean Spaces, Backblaze B2, etc. |

## ⚠️ Error Handling

The library provides comprehensive error types:

```rust
use tinify::{Tinify, TinifyError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("api-key".to_string())?;

    match client.source_from_file("input.png").await {
        Ok(source) => {
            println!("Processing successful");
            // Continue processing...
        }
        Err(TinifyError::FileNotFound { path }) => {
            println!("File not found: {}", path);
        }
        Err(TinifyError::UnsupportedFormat { format }) => {
            println!("Unsupported format: {}", format);
        }
        Err(TinifyError::FileTooLarge { size, max_size }) => {
            println!("File too large: {} bytes (max: {} bytes)", size, max_size);
        }
        Err(TinifyError::QuotaExceeded) => {
            println!("API quota exhausted");
        }
        Err(TinifyError::AccountError { status, message }) => {
            println!("Account error [{}]: {}", status, message);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    Ok(())
}
```

## 📊 Performance Optimization

### Async Concurrent Processing

```rust
use tinify::Tinify;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let mut join_set = JoinSet::new();

    // Process multiple images concurrently
    let files = vec!["image1.png", "image2.jpg", "image3.webp"];

    for (i, file) in files.iter().enumerate() {
        let client = client.clone();
        let file = file.to_string();

        join_set.spawn(async move {
            let source = client.source_from_file(&file).await?;
            let output = format!("compressed_{}.png", i);
            source.to_file(&output).await?;
            Ok::<String, tinify::TinifyError>(output)
        });
    }

    // Wait for all tasks to complete
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(filename)) => println!("✅ Compressed: {}", filename),
            Ok(Err(e)) => println!("❌ Compression failed: {}", e),
            Err(e) => println!("❌ Task error: {}", e),
        }
    }

    Ok(())
}
```

### Batch Processing

```rust
use tinify::{Tinify, ResizeOptions, ResizeMethod};

async fn batch_process_images(
    client: &Tinify,
    input_files: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in input_files {
        // Compress and resize
        let source = client.source_from_file(file).await?;

        let resize_options = ResizeOptions {
            method: ResizeMethod::Fit,
            width: Some(800),
            height: Some(600),
        };

        let mut result = source.resize(resize_options).await?;
        let output = format!("processed_{}", file);
        result.to_file(&output).await?;

        println!("✅ Processed: {} -> {}", file, output);
    }

    Ok(())
}
```

## 🌐 Cloud Storage Integration

### AWS S3 Examples

```rust
use tinify::{Tinify, StoreOptions, S3Options};
use serde_json::json;

// Basic S3 upload
let s3_options = S3Options {
    service: "s3".to_string(),
    aws_access_key_id: "your-access-key".to_string(),
    aws_secret_access_key: "your-secret-key".to_string(),
    region: "us-east-1".to_string(),
    path: "my-bucket/images/compressed.png".to_string(),
    headers: None,
    acl: Some("public-read".to_string()),
};

// S3 upload with custom headers
let s3_options_with_headers = S3Options {
    service: "s3".to_string(),
    aws_access_key_id: "your-access-key".to_string(),
    aws_secret_access_key: "your-secret-key".to_string(),
    region: "us-east-1".to_string(),
    path: "my-bucket/images/compressed.png".to_string(),
    headers: Some(json!({
        "Cache-Control": "public, max-age=31536000",
        "Content-Disposition": "inline; filename=\"optimized.png\""
    })),
    acl: Some("public-read".to_string()),
};

let source = client.source_from_file("input.png").await?;
let result = source.store(StoreOptions::S3(s3_options)).await?;
```

### S3-Compatible Storage

Supports various S3-compatible storage services:

- **MinIO**: Self-hosted object storage
- **DigitalOcean Spaces**: Simple cloud storage
- **Backblaze B2**: Affordable cloud storage
- **Wasabi**: High-performance cloud storage

```rust
// MinIO configuration example
let minio_options = S3Options {
    service: "s3".to_string(),
    aws_access_key_id: "minioadmin".to_string(),
    aws_secret_access_key: "minioadmin".to_string(),
    region: "us-east-1".to_string(),
    path: "test-bucket/compressed.png".to_string(),
    headers: None,
    acl: None,
};
```

## 🎯 Complete Feature Showcase

Check out examples in the `examples/` directory:

- `01_compressing_images.rs` - Basic image compression
- `02_resizing_images.rs` - Image resizing operations
- `03_converting_images.rs` - Format conversion
- `04_preserving_metadata.rs` - Metadata preservation
- `05_saving_to_s3.rs` - AWS S3 storage
- `06_saving_to_gcs.rs` - Google Cloud Storage
- `07_error_handling.rs` - Error handling patterns
- `08_compression_count.rs` - Compression counter tracking
- `09_s3_compatible_storage.rs` - S3-compatible services
- `10_comprehensive_demo.rs` - Complete feature demonstration

Run examples:

```bash
# Basic compression example
cargo run --example 01_compressing_images

# Cloud storage test
export TINIFY_API_KEY="your-api-key"
export AWS_ACCESS_KEY_ID="your-aws-key"
export AWS_SECRET_ACCESS_KEY="your-aws-secret"
cargo run --example 05_saving_to_s3

# Error handling demonstration
cargo run --example 07_error_handling
```

## 🔍 API Quota Management

```rust
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;
    let result = source.to_buffer().await?;

    // Check compression count
    if let Some(count) = result.compression_count() {
        println!("Current API usage: {}", count);

        if count > 450 {
            println!("⚠️ Approaching free quota limit (500/month)");
        }
    }

    Ok(())
}
```

## ⚙️ Environment Setup

### Environment Variables

```bash
# Tinify API configuration
export TINIFY_API_KEY="your-tinify-api-key"

# AWS S3 configuration
export AWS_ACCESS_KEY_ID="your-aws-access-key"
export AWS_SECRET_ACCESS_KEY="your-aws-secret-key"

# Google Cloud Storage configuration
export GCP_ACCESS_TOKEN="your-gcp-access-token"
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
```

### Getting API Key

1. Visit [TinyPNG Developer Page](https://tinypng.com/developers)
2. Register account and verify email
3. Get free API key (500 compressions/month)
4. Upgrade to paid plan for higher quotas

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run doc tests
cargo test --doc

# Run specific example
cargo run --example 01_compressing_images

# Test with real images
cargo run --example test_real_image

# Cloud storage integration tests
./test_cloud_storage.sh
```

## 📋 System Requirements

- **Rust**: 1.70.0 or higher
- **Operating System**: Windows, macOS, Linux
- **Network**: Stable internet connection for TinyPNG API access
- **Memory**: Minimum 100MB available memory for image processing

## 🚨 Limitations and Considerations

### API Limitations

- **Free Quota**: 500 compressions/month
- **File Size**: Maximum 5MB per file
- **Supported Formats**: PNG, JPEG, WebP (input), PNG, JPEG, WebP, AVIF (output)
- **Concurrency**: Recommended max 10 concurrent requests

### Best Practices

1. **API Key Security**: Never hardcode API keys, use environment variables
2. **Error Handling**: Always properly handle network and API errors
3. **Quota Monitoring**: Regularly check API usage to avoid limits
4. **File Validation**: Validate file format and size before upload
5. **Concurrency Control**: Manage concurrent request count appropriately

```rust
// Recommended error handling pattern
match client.source_from_file("input.png").await {
    Ok(source) => {
        // Successful processing
    }
    Err(TinifyError::QuotaExceeded) => {
        // Quota exhausted, stop processing or wait for next month
        eprintln!("API quota exhausted, wait for next month or upgrade plan");
    }
    Err(TinifyError::FileTooLarge { size, max_size }) => {
        // File too large, consider preprocessing
        eprintln!("File too large: {} bytes (max: {})", size, max_size);
    }
    Err(e) => {
        // Other errors, log and possibly retry
        eprintln!("Compression failed: {}", e);
    }
}
```

## 🤝 Contributing

We welcome contributions of all kinds!

### Development Setup

```bash
# Clone repository
git clone https://github.com/raynoryim/tinify.git
cd tinify-rs

# Install dependencies and run tests
cargo test

# Run clippy checks
cargo clippy

# Run formatting
cargo fmt

# Run all checks
cargo check --examples
```

### Submitting PRs

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'feat: add amazing feature'`
4. Push branch: `git push origin feature/amazing-feature`
5. Create Pull Request

### Reporting Issues

Please report bugs or request features in [GitHub Issues](https://github.com/raynoryim/tinify/issues).

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🔗 Related Links

- **Documentation**: [docs.rs/tinify](https://docs.rs/tinify)
- **Crates.io**: [crates.io/crates/tinify](https://crates.io/crates/tinify)
- **TinyPNG API**: [tinypng.com/developers](https://tinypng.com/developers)
- **Issue Tracker**: [GitHub Issues](https://github.com/raynoryim/tinify/issues)

## 🙏 Acknowledgments

- [TinyPNG](https://tinypng.com/) for providing excellent image compression API
- Rust community for amazing libraries and tools
- All contributors and users for their support

---

⭐ If this project helps you, please give us a star!
