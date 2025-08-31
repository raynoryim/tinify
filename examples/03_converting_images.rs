use std::error::Error;
use tinify_rs::{ConvertOptions, ImageFormat, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔄 Tinify-rs: Converting Images Example");
    println!("======================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create test images
    create_test_png("convert_input.png").await?;

    let source = client.source_from_file("convert_input.png").await?;
    println!("✅ Source PNG image loaded");

    // Example 1: Convert to JPEG
    println!("\n🖼️  Example 1: Converting PNG to JPEG");
    let jpeg_options = ConvertOptions {
        format: ImageFormat::Jpeg,
        background: None, // Use default background
    };

    match source.convert(jpeg_options).await {
        Ok(mut result) => {
            result.to_file("converted_to_jpeg.jpg").await?;
            println!("   ✅ Converted to JPEG: converted_to_jpeg.jpg");

            if let Some(content_type) = result.content_type() {
                println!("   📄 Content type: {}", content_type);
            }
            if let Some(size) = result.content_length() {
                println!("   📊 File size: {} bytes", size);
            }
        }
        Err(e) => println!("   ❌ JPEG conversion error: {}", e),
    }

    // Example 2: Convert to JPEG with white background
    println!("\n🎨 Example 2: Converting to JPEG with white background");
    let jpeg_white_options = ConvertOptions {
        format: ImageFormat::Jpeg,
        background: Some("#FFFFFF".to_string()),
    };

    let source2 = client.source_from_file("convert_input.png").await?;
    match source2.convert(jpeg_white_options).await {
        Ok(mut result) => {
            result.to_file("converted_jpeg_white.jpg").await?;
            println!("   ✅ Converted to JPEG with white background: converted_jpeg_white.jpg");
        }
        Err(e) => println!("   ❌ JPEG with background conversion error: {}", e),
    }

    // Example 3: Convert to WebP
    println!("\n🌐 Example 3: Converting to WebP format");
    let webp_options = ConvertOptions {
        format: ImageFormat::WebP,
        background: None,
    };

    let source3 = client.source_from_file("convert_input.png").await?;
    match source3.convert(webp_options).await {
        Ok(mut result) => {
            result.to_file("converted_to_webp.webp").await?;
            println!("   ✅ Converted to WebP: converted_to_webp.webp");

            if let Some(content_type) = result.content_type() {
                println!("   📄 Content type: {}", content_type);
            }
        }
        Err(e) => println!("   ❌ WebP conversion error: {}", e),
    }

    // Example 4: Convert to AVIF (next-gen format)
    println!("\n🚀 Example 4: Converting to AVIF format");
    let avif_options = ConvertOptions {
        format: ImageFormat::Avif,
        background: None,
    };

    let source4 = client.source_from_file("convert_input.png").await?;
    match source4.convert(avif_options).await {
        Ok(mut result) => {
            result.to_file("converted_to_avif.avif").await?;
            println!("   ✅ Converted to AVIF: converted_to_avif.avif");

            if let Some(content_type) = result.content_type() {
                println!("   📄 Content type: {}", content_type);
            }
        }
        Err(e) => {
            println!("   ❌ AVIF conversion error: {}", e);
            println!("   ℹ️  AVIF might not be supported in all regions/plans");
        }
    }

    // Example 5: Convert PNG to PNG (essentially recompress with optimization)
    println!("\n🔧 Example 5: Recompressing PNG format");
    let png_options = ConvertOptions {
        format: ImageFormat::Png,
        background: None,
    };

    let source5 = client.source_from_file("convert_input.png").await?;
    match source5.convert(png_options).await {
        Ok(mut result) => {
            result.to_file("recompressed.png").await?;
            println!("   ✅ Recompressed PNG: recompressed.png");
        }
        Err(e) => println!("   ❌ PNG recompression error: {}", e),
    }

    // Example 6: Convert with custom background color
    println!("\n🎨 Example 6: Converting with custom background colors");
    let custom_bg_options = ConvertOptions {
        format: ImageFormat::Jpeg,
        background: Some("#FF0000".to_string()), // Red background
    };

    let source6 = client.source_from_file("convert_input.png").await?;
    match source6.convert(custom_bg_options).await {
        Ok(mut result) => {
            result.to_file("converted_red_bg.jpg").await?;
            println!("   ✅ Converted with red background: converted_red_bg.jpg");
        }
        Err(e) => println!("   ❌ Custom background conversion error: {}", e),
    }

    println!("\n🎉 Format conversion examples completed!");

    // Clean up test files
    cleanup_files(&[
        "convert_input.png",
        "converted_to_jpeg.jpg",
        "converted_jpeg_white.jpg",
        "converted_to_webp.webp",
        "converted_to_avif.avif",
        "recompressed.png",
        "converted_red_bg.jpg",
    ])
    .await;

    Ok(())
}

async fn create_test_png(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a PNG with transparency for better conversion demonstration
    let png_data = create_transparent_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test PNG with transparency: {}", filename);
    Ok(())
}

fn create_transparent_png_data() -> Vec<u8> {
    // PNG with some transparency - useful for background conversion demos
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, // 32x32 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x73, 0x7A, 0x7A, // RGBA color type
        0xF4, 0x00, 0x00, 0x00, 0x04, 0x67, 0x41, 0x4D, // gAMA chunk
        0x41, 0x00, 0x00, 0xB1, 0x8E, 0x7C, 0xFB, 0x51, 0x93, 0x00, 0x00, 0x00, 0x20, 0x63, 0x48,
        0x52, // cHRM chunk
        0x4D, 0x00, 0x00, 0x7A, 0x25, 0x00, 0x00, 0x80, 0x83, 0x00, 0x00, 0xF9, 0xFF, 0x00, 0x00,
        0x80, 0xE9, 0x00, 0x00, 0x75, 0x30, 0x00, 0x00, 0xEA, 0x60, 0x00, 0x00, 0x3A, 0x98, 0x00,
        0x00, 0x17, 0x6F, 0x92, 0x5F, 0xC5, 0x46, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, 0x54,
        0x78, 0x5E, 0x63, // IDAT chunk
        0xF8, 0xFF, 0xFF, 0x3F, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
        0x03, 0x03, 0x00, 0x1F, 0x90, 0x05, 0xFE, 0x57, 0x96, 0x78, 0xC4, 0x00, 0x00, 0x00, 0x00,
        0x49, 0x45, // IEND chunk
        0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ]
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
