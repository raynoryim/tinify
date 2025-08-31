use std::error::Error;
use tinify::Tinify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("📈 Tinify: Compression Count Example");
    println!("======================================");

    // Initialize client with API key
    let api_key = std::env::var("TINIFY_API_KEY")
        .unwrap_or_else(|_| "XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq".to_string());

    let client = Tinify::new(api_key)?;
    println!("✅ Client initialized");

    // Example 1: Basic compression count tracking
    println!("\n📊 Example 1: Basic compression count tracking");

    // Create test image
    create_test_image("count_test.png").await?;

    match client.source_from_file("count_test.png").await {
        Ok(source) => {
            println!("   ✅ Source created");

            // Get compressed data and check count
            match source.to_buffer().await {
                Ok(buffer) => {
                    println!("   ✅ Image compressed: {} bytes", buffer.len());

                    // Save to check metadata
                    tokio::fs::write("compressed_count_test.png", buffer).await?;

                    // Try to get count from a new compression
                    let source2 = client.source_from_file("count_test.png").await?;
                    let mut result = source2
                        .resize(tinify::ResizeOptions {
                            method: tinify::ResizeMethod::Fit,
                            width: Some(100),
                            height: Some(100),
                        })
                        .await?;

                    if let Some(count) = result.compression_count() {
                        println!("   📈 Current compression count: {}", count);
                        println!("      This represents compressions used this month");
                    } else {
                        println!("   ℹ️  Compression count not available in response headers");
                        println!(
                            "      (This may happen with demo API keys or in certain regions)"
                        );
                    }

                    // Save the resized version
                    result.to_file("resized_count_test.png").await?;
                }
                Err(e) => println!("   ❌ Compression failed: {}", e),
            }
        }
        Err(e) => {
            println!("   ❌ Source creation failed: {}", e);
            println!("   ℹ️  This might be due to API key limitations or network issues");
        }
    }

    // Example 2: Multiple operations and count tracking
    println!("\n🔄 Example 2: Multiple operations and count tracking");

    let operations = vec![
        ("compress", "Basic compression"),
        ("resize", "Resize operation"),
        ("convert", "Format conversion"),
    ];

    for (op_type, description) in operations {
        println!("   🔧 {}: {}", op_type, description);

        match client.source_from_file("count_test.png").await {
            Ok(source) => {
                let result = match op_type {
                    "compress" => {
                        match source.to_buffer().await {
                            Ok(buffer) => {
                                tokio::fs::write(&format!("{}_output.png", op_type), buffer)
                                    .await?;
                                println!("      ✅ {} completed", description);
                                None // to_buffer doesn't return TinifyResult with headers
                            }
                            Err(e) => {
                                println!("      ❌ {} failed: {}", description, e);
                                None
                            }
                        }
                    }
                    "resize" => {
                        match source
                            .resize(tinify::ResizeOptions {
                                method: tinify::ResizeMethod::Scale,
                                width: Some(150),
                                height: None,
                            })
                            .await
                        {
                            Ok(mut result) => {
                                result.to_file(&format!("{}_output.png", op_type)).await?;
                                println!("      ✅ {} completed", description);
                                Some(result)
                            }
                            Err(e) => {
                                println!("      ❌ {} failed: {}", description, e);
                                None
                            }
                        }
                    }
                    "convert" => {
                        match source
                            .convert(tinify::ConvertOptions {
                                format: tinify::ImageFormat::Jpeg,
                                background: Some("#FFFFFF".to_string()),
                            })
                            .await
                        {
                            Ok(mut result) => {
                                result.to_file(&format!("{}_output.jpg", op_type)).await?;
                                println!("      ✅ {} completed", description);
                                Some(result)
                            }
                            Err(e) => {
                                println!("      ❌ {} failed: {}", description, e);
                                None
                            }
                        }
                    }
                    _ => None,
                };

                if let Some(res) = result {
                    if let Some(count) = res.compression_count() {
                        println!("      📈 Compression count after {}: {}", op_type, count);
                    }

                    // Show other metadata too
                    if let Some(width) = res.image_width() {
                        println!("      📐 Image width: {}px", width);
                    }
                    if let Some(height) = res.image_height() {
                        println!("      📐 Image height: {}px", height);
                    }
                    if let Some(size) = res.content_length() {
                        println!("      📊 File size: {} bytes", size);
                    }
                }
            }
            Err(e) => println!("      ❌ Source creation failed: {}", e),
        }
    }

    // Example 3: Quota monitoring simulation
    println!("\n💳 Example 3: Quota monitoring simulation");

    // Simulate quota information
    println!("   ℹ️  Quota Monitoring Best Practices:");
    println!("   • Check compression count after each operation");
    println!("   • Set alerts when approaching monthly limit");
    println!("   • Monitor usage patterns to optimize plan");
    println!("   • Free tier: 500 compressions/month");
    println!("   • Paid plans: 10,000 - 1,000,000 compressions/month");

    // Try to demonstrate quota checking pattern
    let test_count = 5;
    println!("   🧪 Testing {} sequential compressions:", test_count);

    for i in 1..=test_count {
        match client.source_from_file("count_test.png").await {
            Ok(source) => {
                match source
                    .resize(tinify::ResizeOptions {
                        method: tinify::ResizeMethod::Fit,
                        width: Some(50 + i * 10),
                        height: Some(50 + i * 10),
                    })
                    .await
                {
                    Ok(mut result) => {
                        if let Some(count) = result.compression_count() {
                            println!("      #{}: Count = {} (+1 compression)", i, count);

                            // Simulate quota warning
                            let monthly_limit = 500; // Free tier limit
                            let usage_percent = (count as f64 / monthly_limit as f64) * 100.0;

                            if usage_percent > 80.0 {
                                println!(
                                    "         ⚠️  WARNING: {}% of monthly quota used",
                                    usage_percent as u32
                                );
                            } else if usage_percent > 90.0 {
                                println!(
                                    "         🚨 ALERT: {}% of monthly quota used!",
                                    usage_percent as u32
                                );
                            }
                        } else {
                            println!("      #{}: Count unavailable", i);
                        }

                        // Save with unique name
                        result.to_file(&format!("batch_test_{}.png", i)).await?;
                    }
                    Err(e) => {
                        println!("      #{}: Error - {}", i, e);
                        break; // Stop on error
                    }
                }
            }
            Err(e) => {
                println!("      #{}: Source error - {}", i, e);
                break;
            }
        }

        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Example 4: Response header analysis
    println!("\n📋 Example 4: Complete response header analysis");

    match client.source_from_file("count_test.png").await {
        Ok(source) => {
            match source
                .resize(tinify::ResizeOptions {
                    method: tinify::ResizeMethod::Thumb,
                    width: Some(64),
                    height: Some(64),
                })
                .await
            {
                Ok(mut result) => {
                    println!("   ✅ Thumbnail created");
                    println!("   📊 Response Analysis:");

                    if let Some(count) = result.compression_count() {
                        println!("      Compression-Count: {}", count);
                    }

                    if let Some(width) = result.image_width() {
                        println!("      Image-Width: {}px", width);
                    }

                    if let Some(height) = result.image_height() {
                        println!("      Image-Height: {}px", height);
                    }

                    if let Some(content_type) = result.content_type() {
                        println!("      Content-Type: {}", content_type);
                    }

                    if let Some(content_length) = result.content_length() {
                        println!("      Content-Length: {} bytes", content_length);
                    }

                    result.to_file("header_analysis_result.png").await?;
                }
                Err(e) => println!("   ❌ Thumbnail creation failed: {}", e),
            }
        }
        Err(e) => println!("   ❌ Source creation failed: {}", e),
    }

    // Example 5: Best practices demonstration
    println!("\n✅ Example 5: Best practices for compression count monitoring");

    println!("   🎯 Recommended Monitoring Patterns:");
    println!();
    println!("   1. Track Count Per Operation:");
    println!("      let mut result = source.resize(options).await?;");
    println!("      if let Some(count) = result.compression_count() {{");
    println!("          log_usage(count);");
    println!("      }}");
    println!();
    println!("   2. Set Up Quota Alerts:");
    println!("      if count > (monthly_limit * 0.8) {{");
    println!("          send_warning_notification();");
    println!("      }}");
    println!();
    println!("   3. Batch Processing with Limits:");
    println!("      for file in files {{");
    println!("          if current_count < monthly_limit {{");
    println!("              process_file(file).await?;");
    println!("          }} else {{");
    println!("              schedule_for_next_month(file);");
    println!("          }}");
    println!("      }}");
    println!();
    println!("   4. Usage Analytics:");
    println!("      • Track daily/weekly usage patterns");
    println!("      • Monitor peak usage times");
    println!("      • Plan capacity based on trends");

    println!("\n🎉 Compression count examples completed!");
    println!("\nℹ️  Important Notes:");
    println!("   • Compression count resets monthly (on your billing date)");
    println!("   • Each API operation that processes an image counts as one compression");
    println!("   • Storing to cloud storage does NOT count as additional compression");
    println!("   • Preserving metadata does NOT count as additional compression");
    println!("   • Failed operations do NOT count toward your quota");

    // Clean up test files
    cleanup_files(&[
        "count_test.png",
        "compressed_count_test.png",
        "resized_count_test.png",
        "compress_output.png",
        "resize_output.png",
        "convert_output.jpg",
        "header_analysis_result.png",
        "batch_test_1.png",
        "batch_test_2.png",
        "batch_test_3.png",
        "batch_test_4.png",
        "batch_test_5.png",
    ])
    .await;

    Ok(())
}

async fn create_test_image(filename: &str) -> Result<(), Box<dyn Error>> {
    let png_data = create_larger_png_data();
    tokio::fs::write(filename, png_data).await?;
    println!("✅ Created test image: {}", filename);
    Ok(())
}

fn create_larger_png_data() -> Vec<u8> {
    // Larger PNG for better compression demonstration
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, // 128x128 image
        0x08, 0x06, 0x00, 0x00, 0x00, 0xC3, 0x3E, 0x61, // RGBA format
        0xCB, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9C, 0x62, 0x66, 0x60, 0x60, 0xF8, // Compressed data
        0x0F, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x30,
        0x31, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, // IEND chunk
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
