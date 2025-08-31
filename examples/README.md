# Tinify Examples

This directory contains comprehensive examples demonstrating all features of the `tinify` library. All examples are designed to work with the provided API key and demonstrate production-ready usage patterns.

## Quick Start

### Prerequisites

```bash
# Set your Tinify API key (optional - examples use a provided key by default)
export TINIFY_API_KEY="your-api-key-here"

# For cloud storage examples (optional - examples show expected behavior with demo credentials)
export AWS_ACCESS_KEY_ID="your-aws-access-key"
export AWS_SECRET_ACCESS_KEY="your-aws-secret-key"
export GCP_ACCESS_TOKEN="your-gcp-access-token"
```

### Running Examples

```bash
# Run individual examples
cargo run --example 01_compressing_images
cargo run --example 02_resizing_images
cargo run --example 03_converting_images
cargo run --example 04_preserving_metadata
cargo run --example 05_saving_to_s3
cargo run --example 06_saving_to_gcs
cargo run --example 07_error_handling
cargo run --example 08_compression_count
cargo run --example 09_s3_compatible_storage
cargo run --example 10_comprehensive_demo

# Run all examples
cargo run --example 10_comprehensive_demo
```

## Examples Overview

### 1. Basic Image Compression (`01_compressing_images.rs`)

Demonstrates the core functionality of image compression:
- Loading images from files
- Compressing images from memory buffers
- Compressing images from URLs
- Basic error handling

**Key Features:**
```rust
let client = Tinify::new(api_key)?;
let source = client.source_from_file("input.png").await?;
source.to_file("output.png").await?;
```

### 2. Image Resizing (`02_resizing_images.rs`)

Shows all available resizing methods and options:
- **Scale**: Maintains aspect ratio
- **Fit**: Fits within dimensions
- **Cover**: Covers entire area (may crop)
- **Thumb**: Intelligent cropping for thumbnails

**Key Features:**
```rust
let resize_options = ResizeOptions {
    method: ResizeMethod::Fit,
    width: Some(300),
    height: Some(200),
};
let result = source.resize(resize_options).await?;
```

### 3. Format Conversion (`03_converting_images.rs`)

Demonstrates format conversion capabilities:
- PNG to JPEG with background colors
- PNG to WebP for web optimization
- PNG to AVIF for next-generation formats
- Custom background colors for transparency

**Key Features:**
```rust
let convert_options = ConvertOptions {
    format: ImageFormat::Jpeg,
    background: Some("#FFFFFF".to_string()),
};
let result = source.convert(convert_options).await?;
```

### 4. Metadata Preservation (`04_preserving_metadata.rs`)

Shows how to preserve image metadata during compression:
- Copyright information
- Creation date/time
- GPS location data (JPEG only)
- Multiple metadata types

**Key Features:**
```rust
let preserve_options = PreserveOptions {
    preserve: vec![
        PreserveMetadata::Copyright,
        PreserveMetadata::Creation,
        PreserveMetadata::Location,
    ],
};
let result = source.preserve(preserve_options).await?;
```

### 5. Amazon S3 Storage (`05_saving_to_s3.rs`)

Demonstrates direct storage to Amazon S3:
- Basic S3 storage
- Public ACL configuration
- Custom headers (Cache-Control, Expires)
- Multiple AWS regions
- Different path structures

**Key Features:**
```rust
let s3_options = S3Options {
    aws_access_key_id: "your-key".to_string(),
    aws_secret_access_key: "your-secret".to_string(),
    region: "us-east-1".to_string(),
    path: "bucket/path/image.png".to_string(),
    headers: Some(custom_headers),
    acl: Some("public-read".to_string()),
};
source.store(StoreOptions::S3(s3_options)).await?;
```

### 6. Google Cloud Storage (`06_saving_to_gcs.rs`)

Shows direct storage to Google Cloud Storage:
- Basic GCS storage
- Custom metadata headers
- Different bucket structures
- Authentication patterns
- Content-Type handling

**Key Features:**
```rust
let gcs_options = GCSOptions {
    gcp_access_token: "your-token".to_string(),
    path: "bucket/path/image.png".to_string(),
    headers: Some(metadata_headers),
};
source.store(StoreOptions::GCS(gcs_options)).await?;
```

### 7. Error Handling (`07_error_handling.rs`)

Comprehensive error handling demonstration:
- Invalid API key detection
- File not found errors
- Unsupported format errors
- File size limit errors
- Network error handling
- Recovery patterns

**Key Features:**
```rust
match client.source_from_file("nonexistent.png").await {
    Ok(source) => { /* handle success */ },
    Err(TinifyError::FileNotFound { path }) => {
        println!("File not found: {:?}", path);
    },
    Err(e) => { /* handle other errors */ },
}
```

### 8. Compression Count Tracking (`08_compression_count.rs`)

Shows how to monitor API usage and quota:
- Compression count tracking
- Quota monitoring patterns
- Usage analytics
- Batch processing with limits
- Response header analysis

**Key Features:**
```rust
let mut result = source.resize(options).await?;
if let Some(count) = result.compression_count() {
    println!("Current usage: {}", count);
    monitor_quota(count);
}
```

### 9. S3-Compatible Storage (`09_s3_compatible_storage.rs`)

Demonstrates storage to S3-compatible services:
- DigitalOcean Spaces
- Backblaze B2
- Wasabi Hot Cloud Storage
- G-Core Labs
- MinIO (self-hosted)
- Service comparison and pricing

**Key Features:**
```rust
// Works with any S3-compatible service
let do_spaces_options = S3Options {
    aws_access_key_id: "spaces-key".to_string(),
    aws_secret_access_key: "spaces-secret".to_string(),
    region: "nyc3".to_string(), // DigitalOcean region
    path: "my-space/image.png".to_string(),
    // ... other options
};
```

### 10. Comprehensive Demo (`10_comprehensive_demo.rs`)

Complete demonstration of all library features:
- Builder pattern configuration
- All operation types
- Performance monitoring
- Error handling
- Usage statistics
- File cleanup

**Key Features:**
```rust
let client = Tinify::builder()
    .api_key(&api_key)
    .app_identifier("MyApp/1.0")
    .timeout(Duration::from_secs(30))
    .max_retry_attempts(3)
    .requests_per_minute(100)
    .build()?;
```

## Configuration Patterns

### Environment Variables

The examples support these environment variables:

```bash
# Required
TINIFY_API_KEY="your-tinify-api-key"

# For S3/S3-compatible services
AWS_ACCESS_KEY_ID="your-access-key"
AWS_SECRET_ACCESS_KEY="your-secret-key"
DO_SPACES_KEY="your-digitalocean-key"
DO_SPACES_SECRET="your-digitalocean-secret"
B2_APPLICATION_KEY_ID="your-b2-key-id"
B2_APPLICATION_KEY="your-b2-application-key"
WASABI_ACCESS_KEY="your-wasabi-key"
WASABI_SECRET_KEY="your-wasabi-secret"

# For Google Cloud Storage
GCP_ACCESS_TOKEN="your-gcp-access-token"
GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
```

### Builder Pattern Configuration

```rust
use tinify::{Tinify, RetryConfig, RateLimit};
use std::time::Duration;

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
    .api_key("your-api-key")
    .app_identifier("YourApp/1.0")
    .timeout(Duration::from_secs(60))
    .retry_config(retry_config)
    .rate_limit(rate_limit)
    .build()?;
```

## Error Handling Patterns

### Comprehensive Error Matching

```rust
use tinify::TinifyError;

match operation_result {
    Ok(result) => { /* handle success */ },
    Err(TinifyError::InvalidApiKey) => {
        eprintln!("Authentication failed - check your API key");
    },
    Err(TinifyError::FileNotFound { path }) => {
        eprintln!("File not found: {:?}", path);
    },
    Err(TinifyError::FileTooLarge { size, max_size }) => {
        eprintln!("File too large: {} bytes (max: {} bytes)", size, max_size);
    },
    Err(TinifyError::QuotaExceeded { limit, count }) => {
        eprintln!("Monthly quota exceeded: {} / {}", count, limit);
    },
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

### Recovery Patterns

```rust
// Retry pattern for transient errors
let max_retries = 3;
for attempt in 1..=max_retries {
    match client.source_from_file("image.png").await {
        Ok(source) => {
            println!("Success on attempt {}", attempt);
            break;
        },
        Err(TinifyError::NetworkError(_)) if attempt < max_retries => {
            println!("Network error, retrying... ({}/{})", attempt, max_retries);
            tokio::time::sleep(Duration::from_secs(attempt)).await;
        },
        Err(e) => {
            eprintln!("Failed after {} attempts: {}", attempt, e);
            return Err(e.into());
        }
    }
}
```

## Performance Optimization

### Batch Processing

```rust
use futures::future::try_join_all;

// Process multiple images concurrently
let files = vec!["image1.png", "image2.png", "image3.png"];
let futures = files.into_iter().map(|file| async {
    let source = client.source_from_file(file).await?;
    source.to_file(&format!("compressed_{}", file)).await
});

try_join_all(futures).await?;
```

### Memory Management

```rust
// Use streaming for large files
use tokio::fs::File;

let file = File::open("large_image.png").await?;
let source = client.source_from_stream(file, "image/png").await?;
source.to_file("compressed_large.png").await?;
```

## Cloud Storage Best Practices

### S3 Configuration

```rust
use serde_json::json;

// Optimal S3 configuration for web assets
let s3_options = S3Options {
    aws_access_key_id: env::var("AWS_ACCESS_KEY_ID")?,
    aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY")?,
    region: "us-east-1".to_string(),
    path: "images/compressed/image.png".to_string(),
    headers: Some(json!({
        "Cache-Control": "public, max-age=31536000, immutable",
        "Content-Disposition": "inline",
    })),
    acl: Some("public-read".to_string()),
};
```

### GCS Configuration

```rust
// Optimal GCS configuration with metadata
let gcs_options = GCSOptions {
    gcp_access_token: get_access_token().await?,
    path: "images-bucket/compressed/image.png".to_string(),
    headers: Some(json!({
        "Cache-Control": "public, max-age=86400",
        "X-Goog-Meta-Source": "tinify",
        "X-Goog-Meta-Version": "1.0",
    })),
};
```

## Testing Your Setup

Run the comprehensive demo to verify everything is working:

```bash
# This will test all features and provide detailed output
TINIFY_API_KEY="XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq" cargo run --example 10_comprehensive_demo
```

Expected output includes:
- ✅ Client initialization
- ✅ Image compression with size comparison
- ✅ Resize operations with different methods
- ✅ Format conversions
- ✅ Metadata preservation
- ✅ Cloud storage demonstrations (will show expected errors with demo credentials)
- ✅ Error handling verification
- ✅ Usage statistics

## Troubleshooting

### Common Issues

1. **"Image could not be decoded"**: The test PNG data is minimal. For real testing, use actual image files.

2. **Network timeouts**: Increase timeout in client configuration:
   ```rust
   let client = Tinify::builder()
       .timeout(Duration::from_secs(60))
       .build()?;
   ```

3. **Rate limiting**: Reduce request frequency or increase rate limit:
   ```rust
   let client = Tinify::builder()
       .requests_per_minute(50)
       .build()?;
   ```

4. **Cloud storage errors**: These are expected with demo credentials. Use real credentials for actual storage.

### Getting Help

- Check the [Tinify API documentation](https://tinypng.com/developers/reference)
- Review error messages for specific guidance
- Ensure API key is valid and has available quota
- Verify network connectivity to tinify.com

## Next Steps

After running the examples:

1. **Integration**: Incorporate patterns into your application
2. **Configuration**: Set up production environment variables
3. **Monitoring**: Implement compression count tracking
4. **Testing**: Create comprehensive tests for your use cases
5. **Optimization**: Fine-tune settings for your specific requirements

These examples provide a complete foundation for using tinify in production applications with all major features demonstrated and ready for customization.
