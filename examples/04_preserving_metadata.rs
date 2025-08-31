use std::error::Error;
use tinify::{PreserveMetadata, PreserveOptions, Tinify};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🏷️  Tinify: Preserving Metadata Example");
    println!("=========================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Create test image with metadata
    create_test_image_with_metadata("metadata_input.jpg").await?;

    let source = client.source_from_file("metadata_input.jpg").await?;
    println!("✅ Source image with metadata loaded");

    // Example 1: Preserve copyright information
    println!("\n©️  Example 1: Preserving copyright information");
    let copyright_options = PreserveOptions {
        preserve: vec![PreserveMetadata::Copyright],
    };

    match source.preserve(copyright_options).await {
        Ok(mut result) => {
            result.to_file("preserved_copyright.jpg").await?;
            println!("   ✅ Image with preserved copyright saved: preserved_copyright.jpg");

            if let Some(size) = result.content_length() {
                println!("   📊 File size: {} bytes", size);
            }
        }
        Err(e) => println!("   ❌ Copyright preservation error: {}", e),
    }

    // Example 2: Preserve creation date/time
    println!("\n📅 Example 2: Preserving creation date/time");
    let creation_options = PreserveOptions {
        preserve: vec![PreserveMetadata::Creation],
    };

    let source2 = client.source_from_file("metadata_input.jpg").await?;
    match source2.preserve(creation_options).await {
        Ok(mut result) => {
            result.to_file("preserved_creation.jpg").await?;
            println!("   ✅ Image with preserved creation date saved: preserved_creation.jpg");

            if let Some(compression_count) = result.compression_count() {
                println!("   📈 Compression count: {}", compression_count);
            }
        }
        Err(e) => println!("   ❌ Creation date preservation error: {}", e),
    }

    // Example 3: Preserve GPS location data
    println!("\n📍 Example 3: Preserving GPS location data");
    let location_options = PreserveOptions {
        preserve: vec![PreserveMetadata::Location],
    };

    let source3 = client.source_from_file("metadata_input.jpg").await?;
    match source3.preserve(location_options).await {
        Ok(mut result) => {
            result.to_file("preserved_location.jpg").await?;
            println!("   ✅ Image with preserved location data saved: preserved_location.jpg");

            if let Some(content_type) = result.content_type() {
                println!("   📄 Content type: {}", content_type);
            }
        }
        Err(e) => println!("   ❌ Location preservation error: {}", e),
    }

    // Example 4: Preserve multiple metadata types
    println!("\n📋 Example 4: Preserving multiple metadata types");
    let multi_options = PreserveOptions {
        preserve: vec![
            PreserveMetadata::Copyright,
            PreserveMetadata::Creation,
            PreserveMetadata::Location,
        ],
    };

    let source4 = client.source_from_file("metadata_input.jpg").await?;
    match source4.preserve(multi_options).await {
        Ok(mut result) => {
            result.to_file("preserved_all_metadata.jpg").await?;
            println!("   ✅ Image with all metadata preserved: preserved_all_metadata.jpg");

            if let Some(width) = result.image_width() {
                println!("   📐 Image width: {}px", width);
            }
            if let Some(height) = result.image_height() {
                println!("   📐 Image height: {}px", height);
            }
        }
        Err(e) => println!("   ❌ Multiple metadata preservation error: {}", e),
    }

    // Example 5: Compare with non-preserved version
    println!("\n🔍 Example 5: Comparing preserved vs non-preserved");
    let source5 = client.source_from_file("metadata_input.jpg").await?;
    match source5.to_buffer().await {
        Ok(buffer) => {
            tokio::fs::write("no_metadata_preserved.jpg", buffer).await?;
            println!("   ✅ Image without metadata preservation: no_metadata_preserved.jpg");

            // Show file size comparison
            let original_size = tokio::fs::metadata("metadata_input.jpg").await?.len();
            let preserved_size = tokio::fs::metadata("preserved_all_metadata.jpg")
                .await
                .map(|m| m.len())
                .unwrap_or(0);
            let no_preserve_size = tokio::fs::metadata("no_metadata_preserved.jpg")
                .await
                .map(|m| m.len())
                .unwrap_or(0);

            println!("   📊 Size comparison:");
            println!("      Original: {} bytes", original_size);
            println!("      With metadata: {} bytes", preserved_size);
            println!("      Without metadata: {} bytes", no_preserve_size);
        }
        Err(e) => println!("   ❌ Non-preserved version error: {}", e),
    }

    // Example 6: Demonstrate metadata preservation with different formats
    println!("\n🔄 Example 6: Metadata preservation with format conversion");
    use tinify::{ConvertOptions, ImageFormat};

    let convert_options = ConvertOptions {
        format: ImageFormat::Png,
        background: Some("#FFFFFF".to_string()),
    };

    let source6 = client.source_from_file("metadata_input.jpg").await?;
    match source6.convert(convert_options).await {
        Ok(converted_result) => {
            // Now preserve metadata on the converted image
            let source7 = client.source_from_buffer(converted_result.into()).await?;
            let preserve_options = PreserveOptions {
                preserve: vec![PreserveMetadata::Copyright],
            };

            match source7.preserve(preserve_options).await {
                Ok(mut final_result) => {
                    final_result.to_file("converted_with_metadata.png").await?;
                    println!("   ✅ Converted format with preserved metadata: converted_with_metadata.png");
                }
                Err(e) => println!("   ❌ Format conversion + metadata error: {}", e),
            }
        }
        Err(e) => println!("   ❌ Format conversion error: {}", e),
    }

    println!("\n🎉 Metadata preservation examples completed!");
    println!("\nℹ️  Note: Metadata preservation:");
    println!("   • Copyright: EXIF copyright tag, XMP rights tag, Photoshop copyright");
    println!("   • Creation: Original creation date/time");
    println!("   • Location: GPS coordinates (JPEG only)");
    println!("   • Preserving metadata does NOT count as extra compression");

    // Clean up test files
    cleanup_files(&[
        "metadata_input.jpg",
        "preserved_copyright.jpg",
        "preserved_creation.jpg",
        "preserved_location.jpg",
        "preserved_all_metadata.jpg",
        "no_metadata_preserved.jpg",
        "converted_with_metadata.png",
    ])
    .await;

    Ok(())
}

async fn create_test_image_with_metadata(filename: &str) -> Result<(), Box<dyn Error>> {
    // Create a JPEG with some basic EXIF data structure
    let jpeg_data = create_jpeg_with_metadata();
    tokio::fs::write(filename, jpeg_data).await?;
    println!("✅ Created test JPEG with simulated metadata: {}", filename);
    Ok(())
}

fn create_jpeg_with_metadata() -> Vec<u8> {
    // Minimal JPEG with basic EXIF structure
    let mut jpeg_data = Vec::new();

    // JPEG SOI marker
    jpeg_data.extend_from_slice(&[0xFF, 0xD8]);

    // APP1 marker for EXIF
    jpeg_data.extend_from_slice(&[0xFF, 0xE1]);
    jpeg_data.extend_from_slice(&[0x00, 0x1C]); // Length
    jpeg_data.extend_from_slice(b"Exif\0\0"); // EXIF header

    // Basic TIFF header
    jpeg_data.extend_from_slice(&[0x49, 0x49]); // Little endian
    jpeg_data.extend_from_slice(&[0x2A, 0x00]); // TIFF magic
    jpeg_data.extend_from_slice(&[0x08, 0x00, 0x00, 0x00]); // Offset to IFD

    // Very basic IFD with one entry
    jpeg_data.extend_from_slice(&[0x01, 0x00]); // Number of entries
    jpeg_data.extend_from_slice(&[
        0x0F, 0x01, 0x02, 0x00, 0x04, 0x00, 0x00, 0x00, 0x54, 0x65, 0x73, 0x74,
    ]); // Basic tag

    // DQT (quantization table)
    jpeg_data.extend_from_slice(&[0xFF, 0xDB]);
    jpeg_data.extend_from_slice(&[0x00, 0x43]); // Length
    jpeg_data.extend_from_slice(&[0x00]); // Table ID

    // Minimal quantization table
    let q_table = [16u8; 64]; // Simple quantization values
    jpeg_data.extend_from_slice(&q_table);

    // SOF0 marker (Start of Frame)
    jpeg_data.extend_from_slice(&[0xFF, 0xC0]);
    jpeg_data.extend_from_slice(&[0x00, 0x11]); // Length
    jpeg_data.extend_from_slice(&[0x08]); // Precision
    jpeg_data.extend_from_slice(&[0x00, 0x10]); // Height = 16
    jpeg_data.extend_from_slice(&[0x00, 0x10]); // Width = 16
    jpeg_data.extend_from_slice(&[0x01]); // Number of components
    jpeg_data.extend_from_slice(&[0x01, 0x11, 0x00]); // Component info

    // DHT (Huffman table) - minimal
    jpeg_data.extend_from_slice(&[0xFF, 0xC4]);
    jpeg_data.extend_from_slice(&[0x00, 0x1F]); // Length
    jpeg_data.extend_from_slice(&[0x00]); // Table info

    // Minimal Huffman table
    let huffman_lengths = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
    jpeg_data.extend_from_slice(&huffman_lengths);
    let huffman_values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    jpeg_data.extend_from_slice(&huffman_values);

    // SOS (Start of Scan)
    jpeg_data.extend_from_slice(&[0xFF, 0xDA]);
    jpeg_data.extend_from_slice(&[0x00, 0x0C]); // Length
    jpeg_data.extend_from_slice(&[0x01]); // Number of components
    jpeg_data.extend_from_slice(&[0x01, 0x00]); // Component info
    jpeg_data.extend_from_slice(&[0x00, 0x3F, 0x00]); // Start, End, Ah/Al

    // Minimal compressed data
    jpeg_data.extend_from_slice(&[0xFF, 0x00]); // Escaped 0xFF

    // EOI marker
    jpeg_data.extend_from_slice(&[0xFF, 0xD9]);

    jpeg_data
}

async fn cleanup_files(files: &[&str]) {
    for file in files {
        if tokio::fs::metadata(file).await.is_ok() {
            let _ = tokio::fs::remove_file(file).await;
        }
    }
}
