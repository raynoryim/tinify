# Tinify-rs

[![Crates.io](https://img.shields.io/crates/v/tinify-rs.svg)](https://crates.io/crates/tinify-rs)
[![Documentation](https://docs.rs/tinify-rs/badge.svg)](https://docs.rs/tinify-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/raynoryim/tinify/workflows/CI/badge.svg)](https://github.com/raynoryim/tinify/actions)

[English](README.md) | **中文**

一个高性能的 Rust 图片压缩和优化库，基于 [TinyPNG API](https://tinypng.com/developers) 构建。提供异步支持、智能重试机制、速率限制和云存储集成。

## ✨ 特性

- 🖼️ **智能压缩**: 无损质量的 PNG/JPEG/WebP/AVIF 图片压缩
- 📏 **尺寸调整**: 多种调整方法（scale/fit/cover/thumb）
- 🔄 **格式转换**: 支持主流图片格式之间的转换
- 📊 **元数据保留**: 可选保留版权、创建时间、位置等信息
- ☁️ **云存储集成**: 直接保存到 AWS S3、Google Cloud Storage
- 🚀 **高性能异步**: 基于 tokio 的异步 I/O，支持并发处理
- 🛡️ **强类型安全**: 完整的 Rust 类型系统和错误处理
- ⚡ **智能重试**: 内置指数退避重试机制和速率限制
- 📦 **零配置**: 开箱即用，无需复杂配置

## 📦 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tinify-rs = "0.3.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 快速开始

### 基本用法

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let client = Tinify::new("your-api-key".to_string())?;

    // 压缩图片
    let source = client.source_from_file("input.png").await?;
    source.to_file("output.png").await?;

    println!("图片压缩完成！");
    Ok(())
}
```

### 高级配置

```rust
use tinify_rs::Tinify;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用构建器模式进行高级配置
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

## 📖 详细示例

### 图片尺寸调整

```rust
use tinify_rs::{Tinify, ResizeOptions, ResizeMethod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // 配置调整选项
    let resize_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(300),
        height: Some(200),
    };

    // 调整尺寸
    let mut result = source.resize(resize_options).await?;
    result.to_file("resized.png").await?;

    // 获取图片信息
    if let Some(width) = result.image_width() {
        println!("调整后宽度: {} 像素", width);
    }

    Ok(())
}
```

### 格式转换

```rust
use tinify_rs::{Tinify, ConvertOptions, ImageFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // 转换为 WebP 格式
    let convert_options = ConvertOptions {
        format: ImageFormat::WebP,
        background: Some("#FFFFFF".to_string()),
    };

    let mut result = source.convert(convert_options).await?;
    result.to_file("output.webp").await?;

    Ok(())
}
```

### 元数据保留

```rust
use tinify_rs::{Tinify, PreserveOptions, PreserveMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.jpg").await?;

    // 保留版权和创建时间信息
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

### AWS S3 云存储

```rust
use tinify_rs::{Tinify, StoreOptions, S3Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // 配置 S3 存储选项
    let s3_options = S3Options {
        service: "s3".to_string(),
        aws_access_key_id: "your-access-key".to_string(),
        aws_secret_access_key: "your-secret-key".to_string(),
        region: "us-east-1".to_string(),
        path: "my-bucket/images/compressed.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    // 直接保存到 S3
    let result = source.store(StoreOptions::S3(s3_options)).await?;

    if let Some(count) = result.compression_count() {
        println!("API 使用次数: {}", count);
    }

    Ok(())
}
```

### Google Cloud Storage

```rust
use tinify_rs::{Tinify, StoreOptions, GCSOptions};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;

    // 配置 GCS 存储选项
    let gcs_options = GCSOptions {
        service: "gcs".to_string(),
        gcp_access_token: "your-access-token".to_string(),
        path: "my-bucket/images/compressed.png".to_string(),
        headers: Some(json!({
            "Cache-Control": "public, max-age=31536000",
            "X-Goog-Meta-Source": "tinify-rs"
        })),
    };

    // 直接保存到 GCS
    let result = source.store(StoreOptions::GCS(gcs_options)).await?;

    Ok(())
}
```

### 从 URL 处理图片

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;

    // 从 URL 加载图片
    let source = client.source_from_url("https://example.com/image.jpg").await?;
    source.to_file("compressed.jpg").await?;

    Ok(())
}
```

### 从内存缓冲区处理

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;

    // 从内存中的字节数据创建源
    let image_data = std::fs::read("input.png")?;
    let source = client.source_from_buffer(image_data).await?;

    // 获取压缩后的字节数据
    let compressed_data = source.to_buffer().await?;
    std::fs::write("output.png", compressed_data)?;

    Ok(())
}
```

## 🔧 API 参考

### 调整方法 (ResizeMethod)

| 方法 | 描述 | 用途 |
|------|------|------|
| `Scale` | 按比例缩放 | 精确控制宽度或高度 |
| `Fit` | 适应尺寸（保持宽高比） | 创建指定尺寸内的最大图片 |
| `Cover` | 覆盖尺寸（可能裁剪） | 填满指定尺寸，保持宽高比 |
| `Thumb` | 智能缩略图 | 自动识别重要区域生成缩略图 |

### 支持的图片格式

| 格式 | 输入支持 | 输出支持 | 描述 |
|------|----------|----------|------|
| PNG | ✅ | ✅ | 无损压缩，支持透明 |
| JPEG | ✅ | ✅ | 有损压缩，适合照片 |
| WebP | ✅ | ✅ | 现代格式，更小体积 |
| AVIF | ❌ | ✅ | 下一代格式，最优压缩 |

### 云存储支持

| 服务 | 支持状态 | 说明 |
|------|----------|------|
| AWS S3 | ✅ | 完整支持，包括自定义头部和 ACL |
| Google Cloud Storage | ✅ | 完整支持，包括元数据 |
| 兼容 S3 的服务 | ✅ | MinIO、DigitalOcean Spaces、Backblaze B2 等 |

## ⚠️ 错误处理

库提供了完整的错误类型体系：

```rust
use tinify_rs::{Tinify, TinifyError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("api-key".to_string())?;

    match client.source_from_file("input.png").await {
        Ok(source) => {
            println!("处理成功");
            // 继续处理...
        }
        Err(TinifyError::FileNotFound { path }) => {
            println!("文件未找到: {}", path);
        }
        Err(TinifyError::UnsupportedFormat { format }) => {
            println!("不支持的格式: {}", format);
        }
        Err(TinifyError::FileTooLarge { size, max_size }) => {
            println!("文件过大: {} 字节 (最大: {} 字节)", size, max_size);
        }
        Err(TinifyError::QuotaExceeded) => {
            println!("API 配额已用完");
        }
        Err(TinifyError::AccountError { status, message }) => {
            println!("账户错误 [{}]: {}", status, message);
        }
        Err(e) => {
            println!("其他错误: {}", e);
        }
    }

    Ok(())
}
```

## 📊 性能优化

### 异步并发处理

```rust
use tinify_rs::Tinify;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let mut join_set = JoinSet::new();

    // 并发处理多个图片
    let files = vec!["image1.png", "image2.jpg", "image3.webp"];

    for (i, file) in files.iter().enumerate() {
        let client = client.clone();
        let file = file.to_string();

        join_set.spawn(async move {
            let source = client.source_from_file(&file).await?;
            let output = format!("compressed_{}.png", i);
            source.to_file(&output).await?;
            Ok::<String, tinify_rs::TinifyError>(output)
        });
    }

    // 等待所有任务完成
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(filename)) => println!("✅ 压缩完成: {}", filename),
            Ok(Err(e)) => println!("❌ 压缩失败: {}", e),
            Err(e) => println!("❌ 任务错误: {}", e),
        }
    }

    Ok(())
}
```

### 批量处理模式

```rust
use tinify_rs::{Tinify, ResizeOptions, ResizeMethod};

async fn batch_process_images(
    client: &Tinify,
    input_files: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in input_files {
        // 压缩并调整尺寸
        let source = client.source_from_file(file).await?;

        let resize_options = ResizeOptions {
            method: ResizeMethod::Fit,
            width: Some(800),
            height: Some(600),
        };

        let mut result = source.resize(resize_options).await?;
        let output = format!("processed_{}", file);
        result.to_file(&output).await?;

        println!("✅ 处理完成: {} -> {}", file, output);
    }

    Ok(())
}
```

## 🌐 云存储集成

### AWS S3 示例

```rust
use tinify_rs::{Tinify, StoreOptions, S3Options};
use serde_json::json;

// 基本 S3 上传
let s3_options = S3Options {
    service: "s3".to_string(),
    aws_access_key_id: "your-access-key".to_string(),
    aws_secret_access_key: "your-secret-key".to_string(),
    region: "us-east-1".to_string(),
    path: "my-bucket/images/compressed.png".to_string(),
    headers: None,
    acl: Some("public-read".to_string()),
};

// 带自定义头部的 S3 上传
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

### S3 兼容存储

支持多种 S3 兼容的存储服务：

- **MinIO**: 自托管对象存储
- **DigitalOcean Spaces**: 简单的云存储
- **Backblaze B2**: 经济实惠的云存储
- **Wasabi**: 高性能云存储

```rust
// MinIO 配置示例
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

## 🎯 完整功能展示

查看 `examples/` 目录中的示例：

- `01_compressing_images.rs` - 基本图片压缩
- `02_resizing_images.rs` - 图片尺寸调整
- `03_converting_images.rs` - 格式转换
- `04_preserving_metadata.rs` - 元数据保留
- `05_saving_to_s3.rs` - AWS S3 存储
- `06_saving_to_gcs.rs` - Google Cloud Storage
- `07_error_handling.rs` - 错误处理
- `08_compression_count.rs` - 压缩计数器
- `09_s3_compatible_storage.rs` - S3 兼容存储
- `10_comprehensive_demo.rs` - 综合功能演示

运行示例：

```bash
# 基本压缩示例
cargo run --example 01_compressing_images

# 云存储测试
export TINIFY_API_KEY="your-api-key"
export AWS_ACCESS_KEY_ID="your-aws-key"
export AWS_SECRET_ACCESS_KEY="your-aws-secret"
cargo run --example 05_saving_to_s3

# 错误处理演示
cargo run --example 07_error_handling
```

## 🔍 API 配额管理

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Tinify::new("your-api-key".to_string())?;
    let source = client.source_from_file("input.png").await?;
    let result = source.to_buffer().await?;

    // 检查压缩计数
    if let Some(count) = result.compression_count() {
        println!("当前 API 使用次数: {}", count);

        if count > 450 {
            println!("⚠️ 接近免费配额限制 (500/月)");
        }
    }

    Ok(())
}
```

## ⚙️ 环境配置

### 环境变量

```bash
# Tinify API 配置
export TINIFY_API_KEY="your-tinify-api-key"

# AWS S3 配置
export AWS_ACCESS_KEY_ID="your-aws-access-key"
export AWS_SECRET_ACCESS_KEY="your-aws-secret-key"

# Google Cloud Storage 配置
export GCP_ACCESS_TOKEN="your-gcp-access-token"
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
```

### 获取 API Key

1. 访问 [TinyPNG 开发者页面](https://tinypng.com/developers)
2. 注册账户并验证邮箱
3. 获取免费 API key（每月 500 次压缩）
4. 升级到付费计划获得更高配额

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行文档测试
cargo test --doc

# 运行特定示例
cargo run --example 01_compressing_images

# 使用真实图片测试
cargo run --example test_real_image

# 云存储集成测试
./test_cloud_storage.sh
```

## 📋 系统要求

- **Rust**: 1.70.0 或更高版本
- **操作系统**: Windows、macOS、Linux
- **网络**: 稳定的互联网连接访问 TinyPNG API
- **内存**: 建议至少 100MB 可用内存用于图片处理

## 🚨 限制和注意事项

### API 限制

- **免费配额**: 500 次压缩/月
- **文件大小**: 最大 5MB
- **支持格式**: PNG, JPEG, WebP（输入），PNG, JPEG, WebP, AVIF（输出）
- **并发限制**: 建议不超过 10 个并发请求

### 最佳实践

1. **API Key 安全**: 不要在代码中硬编码 API key，使用环境变量
2. **错误处理**: 始终正确处理网络错误和 API 错误
3. **配额监控**: 定期检查 API 使用量，避免超出限制
4. **文件验证**: 上传前验证文件格式和大小
5. **并发控制**: 合理控制并发请求数量

```rust
// 推荐的错误处理模式
match client.source_from_file("input.png").await {
    Ok(source) => {
        // 成功处理
    }
    Err(TinifyError::QuotaExceeded) => {
        // 配额用完，停止处理或等待下个月
        eprintln!("API 配额已用完，请等待下个月或升级计划");
    }
    Err(TinifyError::FileTooLarge { size, max_size }) => {
        // 文件过大，考虑预处理
        eprintln!("文件过大: {} 字节 (最大: {})", size, max_size);
    }
    Err(e) => {
        // 其他错误，记录并可能重试
        eprintln!("压缩失败: {}", e);
    }
}
```

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 开发设置

```bash
# 克隆仓库
git clone https://github.com/raynoryim/tinify.git
cd tinify-rs

# 安装依赖并运行测试
cargo test

# 运行 clippy 检查
cargo clippy

# 运行格式化
cargo fmt

# 运行所有检查
cargo check --examples
```

### 提交 PR

1. Fork 仓库
2. 创建功能分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'feat: add amazing feature'`
4. 推送分支: `git push origin feature/amazing-feature`
5. 创建 Pull Request

### 报告问题

请在 [GitHub Issues](https://github.com/raynoryim/tinify/issues) 中报告 bug 或提出功能请求。

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🔗 相关链接

- **文档**: [docs.rs/tinify-rs](https://docs.rs/tinify-rs)
- **Crates.io**: [crates.io/crates/tinify-rs](https://crates.io/crates/tinify-rs)
- **TinyPNG API**: [tinypng.com/developers](https://tinypng.com/developers)
- **问题报告**: [GitHub Issues](https://github.com/raynoryim/tinify/issues)

## 🙏 致谢

- [TinyPNG](https://tinypng.com/) 提供优秀的图片压缩 API
- Rust 社区提供的优秀库和工具
- 所有贡献者和使用者的支持

---

⭐ 如果这个项目对你有帮助，请给我们一个 star！
