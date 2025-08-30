# Tinify-rs

一个用于图片压缩和优化的 Rust 库，基于 [TinyPNG API](https://tinypng.com/developers) 构建。

## 项目简介

Tinify-rs 是一个高性能的 Rust 库，提供了简单易用的 API 来压缩和优化图片。它支持多种图片格式，包括 PNG、JPEG、WebP 和 AVIF，并提供了丰富的图片处理功能，如调整大小、格式转换、元数据保留等。

## 主要功能

- 🖼️ **图片压缩**: 智能压缩图片，显著减小文件大小
- 📏 **尺寸调整**: 支持多种调整方法（缩放、适应、覆盖、缩略图）
- 🔄 **格式转换**: 在 PNG、JPEG、WebP、AVIF 之间转换
- 📊 **元数据保留**: 可选择保留版权、创建时间、位置等元数据
- ☁️ **云存储**: 支持直接保存到 AWS S3 和 Google Cloud Storage
- 🚀 **异步支持**: 基于 tokio 的异步操作，高性能处理
- 🛡️ **错误处理**: 完善的错误类型和错误处理机制

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tinify-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 快速开始

### 1. 设置 API Key

首先需要在 [TinyPNG](https://tinypng.com/developers) 注册并获取 API key。

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置你的 API key
    Tinify::set_key("your-api-key-here".to_string()).await?;

    Ok(())
}
```

### 2. 基本使用

#### 从文件压缩图片

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置 API key
    Tinify::set_key("your-api-key-here".to_string()).await?;

    // 从文件压缩图片
    let source = Tinify::from_file("./input.png").await?;

    // 保存压缩后的图片
    source.to_file("./output.png").await?;

    println!("图片压缩完成！");
    Ok(())
}
```

#### 从 URL 压缩图片

```rust
use tinify_rs::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tinify::set_key("your-api-key-here".to_string()).await?;

    // 从 URL 压缩图片
    let source = Tinify::from_url("https://example.com/image.jpg").await?;

    // 保存到文件
    source.to_file("./compressed.jpg").await?;

    Ok(())
}
```

#### 调整图片尺寸

```rust
use tinify_rs::{Tinify, ResizeOptions, ResizeMethod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tinify::set_key("your-api-key-here".to_string()).await?;

    let source = Tinify::from_file("./input.png").await?;

    // 创建调整选项
    let resize_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(300),
        height: Some(200),
    };

    // 调整尺寸并保存
    let mut result = source.resize(resize_options).await?;
    result.to_file("./resized.png").await?;

    // 获取调整后的图片信息
    if let Some(width) = result.image_width() {
        println!("调整后的宽度: {}", width);
    }
    if let Some(height) = result.image_height() {
        println!("调整后的高度: {}", height);
    }

    Ok(())
}
```

#### 格式转换

```rust
use tinify_rs::{Tinify, ConvertOptions, ImageFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tinify::set_key("your-api-key-here".to_string()).await?;

    let source = Tinify::from_file("./input.png").await?;

    // 转换为 WebP 格式
    let convert_options = ConvertOptions {
        format: vec![ImageFormat::WebP],
        background: None,
    };

    let mut result = source.convert(convert_options).await?;
    result.to_file("./output.webp").await?;

    Ok(())
}
```

#### 保留元数据

```rust
use tinify_rs::{Tinify, PreserveOptions, PreserveMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tinify::set_key("your-api-key-here".to_string()).await?;

    let source = Tinify::from_file("./input.jpg").await?;

    // 保留版权和位置信息
    let preserve_options = PreserveOptions {
        preserve: vec![
            PreserveMetadata::Copyright,
            PreserveMetadata::Location,
        ],
    };

    let mut result = source.preserve(preserve_options).await?;
    result.to_file("./preserved.jpg").await?;

    Ok(())
}
```

#### 保存到云存储

```rust
use tinify_rs::{Tinify, StoreOptions, S3Options};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tinify::set_key("your-api-key-here".to_string()).await?;

    let source = Tinify::from_file("./input.png").await?;

    // 配置 S3 存储选项
    let s3_options = S3Options {
        aws_access_key_id: "your-aws-key".to_string(),
        aws_secret_access_key: "your-aws-secret".to_string(),
        region: "us-east-1".to_string(),
        path: "images/compressed.png".to_string(),
        headers: None,
        acl: Some("public-read".to_string()),
    };

    let store_options = StoreOptions::S3(s3_options);

    // 直接保存到 S3
    let _result = source.store(store_options).await?;

    println!("图片已保存到 S3！");
    Ok(())
}
```

## API 参考

### 核心类型

#### `Tinify`

主要的客户端类，提供静态方法进行图片处理。

#### `Source`

表示一个已上传的图片源，提供各种转换操作。

#### `TinifyResult`

表示转换操作的结果，包含压缩后的图片数据和元信息。

### 调整方法

- `ResizeMethod::Scale`: 按比例缩放
- `ResizeMethod::Fit`: 适应指定尺寸，保持宽高比
- `ResizeMethod::Cover`: 覆盖指定尺寸，可能裁剪
- `ResizeMethod::Thumb`: 创建缩略图

### 支持的格式

- `ImageFormat::Png`: PNG 格式
- `ImageFormat::Jpeg`: JPEG 格式
- `ImageFormat::WebP`: WebP 格式
- `ImageFormat::Avif`: AVIF 格式

### 错误处理

库提供了详细的错误类型：

```rust
use tinify_rs::TinifyError;

match result {
    Ok(_) => println!("操作成功"),
    Err(TinifyError::AccountError { message, .. }) => {
        println!("账户错误: {}", message);
    }
    Err(TinifyError::ClientError { message, .. }) => {
        println!("客户端错误: {}", message);
    }
    Err(TinifyError::ServerError { message, .. }) => {
        println!("服务器错误: {}", message);
    }
    Err(e) => println!("其他错误: {:?}", e),
}
```

## 性能优化

- 使用异步 I/O 操作，支持高并发处理
- 智能的 HTTP 客户端复用
- 最小化内存分配
- 支持流式处理大文件

## 限制和注意事项

1. **API 限制**: TinyPNG 对免费账户有每月 500 次压缩的限制
2. **文件大小**: 单个文件最大支持 5MB
3. **并发限制**: 建议控制并发请求数量，避免触发 API 限制
4. **网络依赖**: 需要稳定的网络连接

## 示例项目

查看 `src/lib.rs` 中的测试用例，了解更多使用示例：

```rust
#[tokio::test]
async fn test_from_file() {
    Tinify::set_key("your-api-key".to_string()).await.unwrap();
    let result = Tinify::from_file("./test_file.png").await;
    assert!(result.is_ok());
}
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 相关链接

- [TinyPNG API 文档](https://tinypng.com/developers)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Tokio 异步运行时](https://tokio.rs/)
