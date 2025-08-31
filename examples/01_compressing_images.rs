use std::error::Error;
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🗜️  Tinify: Compressing Images Example");
    println!("==========================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Example 1: Compress from file
    println!("\n📁 Example 1: Compressing from file");

    // Create a test image file if it doesn't exist
    create_test_image("test_input.png").await?;

    match client.source_from_file("test_input.png").await {
        Ok(source) => {
            println!("   ✅ Source created from file");

            // Save compressed image
            source.to_file("compressed_output.png").await?;
            println!("   ✅ Compressed image saved to compressed_output.png");

            // Get image data to buffer
            let client2 = Tinify::new(
                std::env::var("TINIFY_API_KEY")
                    .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string()),
            )?;
            let source2 = client2.source_from_file("test_input.png").await?;
            let buffer = source2.to_buffer().await?;
            println!("   ✅ Image data loaded to buffer: {} bytes", buffer.len());
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
        }
    }

    // Example 2: Compress from buffer
    println!("\n💾 Example 2: Compressing from buffer");

    let image_data = tokio::fs::read("test_input.png")
        .await
        .unwrap_or_else(|_| create_dummy_png_data());

    match client.source_from_buffer(image_data).await {
        Ok(source) => {
            println!("   ✅ Source created from buffer");
            source.to_file("buffer_compressed.png").await?;
            println!("   ✅ Buffer-compressed image saved");
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
        }
    }

    // Example 3: Compress from URL
    println!("\n🌐 Example 3: Compressing from URL");

    let test_url = "https://tinypng.com/images/panda-happy.png";
    match client.source_from_url(test_url).await {
        Ok(source) => {
            println!("   ✅ Source created from URL");
            source.to_file("url_compressed.png").await?;
            println!("   ✅ URL-compressed image saved");
        }
        Err(e) => {
            println!("   ❌ Error compressing from URL: {}", e);
            println!("   ℹ️  This might be due to network restrictions or API limits");
        }
    }

    println!("\n🎉 Compression examples completed!");

    // Clean up test files
    cleanup_files(&[
        "test_input.png",
        "compressed_output.png",
        "buffer_compressed.png",
        "url_compressed.png",
    ])
    .await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a simple PNG file for testing
    let png_data = create_dummy_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("   ✅ Created test image: {}", filename);
    Ok(())
}

fn create_dummy_png_data() -> Vec<u8> {
    // Minimal valid PNG file (1x1 transparent pixel)
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, // bit depth, color type, etc.
        0x89, 0x00, 0x00, 0x00, 0x0B, 0x49, 0x44, 0x41, // IDAT chunk start
        0x54, 0x78, 0x9C, 0x62, 0x00, 0x02, 0x00, 0x00, // compressed data
        0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, // compressed data end
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, // IEND chunk
        0x42, 0x60, 0x82,
    ]
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
