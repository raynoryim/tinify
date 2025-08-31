use std::error::Error;
use tinify::{ResizeMethod, ResizeOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("📏 Tinify: Resizing Images Example");
    println!("====================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create a test image
    create_test_image("resize_input.png").await?;

    let source = client.source_from_file("resize_input.png").await?;
    println!("✅ Source image loaded");

    // Example 1: Scale method - maintains aspect ratio
    println!("\n🔄 Example 1: Scale method (maintains aspect ratio)");
    let scale_options = ResizeOptions {
        method: ResizeMethod::Scale,
        width: Some(150),
        height: None, // Let it maintain aspect ratio
    };

    match source.resize(scale_options).await {
        Ok(mut result) => {
            result.to_file("resized_scale.png").await?;
            println!("   ✅ Scaled image saved to resized_scale.png");

            if let Some(width) = result.image_width() {
                println!("   📐 New width: {}px", width);
            }
            if let Some(height) = result.image_height() {
                println!("   📐 New height: {}px", height);
            }
        }
        Err(e) => println!("   ❌ Scale resize error: {}", e),
    }

    // Example 2: Fit method - fits within dimensions
    println!("\n📦 Example 2: Fit method (fits within dimensions)");
    let fit_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(200),
        height: Some(200),
    };

    let source2 = client.source_from_file("resize_input.png").await?;
    match source2.resize(fit_options).await {
        Ok(mut result) => {
            result.to_file("resized_fit.png").await?;
            println!("   ✅ Fit-resized image saved to resized_fit.png");

            if let Some(width) = result.image_width() {
                println!("   📐 Fit width: {}px", width);
            }
            if let Some(height) = result.image_height() {
                println!("   📐 Fit height: {}px", height);
            }
        }
        Err(e) => println!("   ❌ Fit resize error: {}", e),
    }

    // Example 3: Cover method - covers entire area
    println!("\n🎯 Example 3: Cover method (covers entire area, may crop)");
    let cover_options = ResizeOptions {
        method: ResizeMethod::Cover,
        width: Some(100),
        height: Some(100),
    };

    let source3 = client.source_from_file("resize_input.png").await?;
    match source3.resize(cover_options).await {
        Ok(mut result) => {
            result.to_file("resized_cover.png").await?;
            println!("   ✅ Cover-resized image saved to resized_cover.png");

            if let Some(width) = result.image_width() {
                println!("   📐 Cover width: {}px", width);
            }
            if let Some(height) = result.image_height() {
                println!("   📐 Cover height: {}px", height);
            }
        }
        Err(e) => println!("   ❌ Cover resize error: {}", e),
    }

    // Example 4: Thumb method - for thumbnails
    println!("\n🖼️  Example 4: Thumb method (intelligent cropping for thumbnails)");
    let thumb_options = ResizeOptions {
        method: ResizeMethod::Thumb,
        width: Some(80),
        height: Some(80),
    };

    let source4 = client.source_from_file("resize_input.png").await?;
    match source4.resize(thumb_options).await {
        Ok(mut result) => {
            result.to_file("resized_thumb.png").await?;
            println!("   ✅ Thumbnail saved to resized_thumb.png");

            if let Some(width) = result.image_width() {
                println!("   📐 Thumb width: {}px", width);
            }
            if let Some(height) = result.image_height() {
                println!("   📐 Thumb height: {}px", height);
            }
        }
        Err(e) => println!("   ❌ Thumb resize error: {}", e),
    }

    // Example 5: Error handling for invalid dimensions
    println!("\n⚠️  Example 5: Error handling for invalid dimensions");
    let invalid_options = ResizeOptions {
        method: ResizeMethod::Fit,
        width: Some(0), // Invalid dimension
        height: Some(100),
    };

    let source5 = client.source_from_file("resize_input.png").await?;
    match source5.resize(invalid_options).await {
        Ok(_) => println!("   ❌ Unexpected success with invalid dimensions"),
        Err(e) => println!("   ✅ Correctly caught invalid dimensions: {}", e),
    }

    println!("\n🎉 Resize examples completed!");

    // Clean up test files
    cleanup_files(&[
        "resize_input.png",
        "resized_scale.png",
        "resized_fit.png",
        "resized_cover.png",
        "resized_thumb.png",
    ])
    .await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a larger PNG for better resize demonstration
    let png_data = create_test_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_test_png_data() -> Vec<u8> {
    // Create a minimal but larger PNG (this is still a 1x1 pixel, but demonstrates the concept)
    // In a real scenario, you'd want a proper test image
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x64, // 100x100 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x70, 0xE2, 0x95, // bit depth, color type, etc.
        0x25, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk start
        0x54, 0x78, 0x9C, 0x62, 0xF8, 0x0F, 0x00, 0x00, // minimal compressed data
        0x00, 0xFF, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // for a colored square
        0x00, 0x00, 0x00, 0x1C, 0x30, 0x31, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
        0x42, // IEND chunk
        0x60, 0x82,
    ]
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
