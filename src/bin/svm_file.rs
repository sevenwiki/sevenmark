use sevenmark::sevenmark::core::parse_document_with_preprocessing;
use std::fs;
use std::time::Instant;

fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    let start_time = Instant::now();
    let (result, preprocess_info) = parse_document_with_preprocessing(&input_content);
    let duration = start_time.elapsed();

    println!("Parsed {} elements in {:?}", result.len(), duration);

    // Print preprocessing information
    println!("\n=== Preprocessing Info ===");
    println!("Includes found: {:?}", preprocess_info.includes);
    println!("Categories found: {:?}", preprocess_info.categories);
    println!("Media found {:?}", preprocess_info.media);
    println!("Redirect found {:?}", preprocess_info.redirect);

    if let Some(redirect) = &preprocess_info.redirect {
        println!("Redirect to: {}", redirect);
    }
    println!("Media URLs found: {:?}", preprocess_info.media);

    let json_output = serde_json::to_string_pretty(&result).unwrap();
    // println!("JSON Output:\n{}", json_output);

    fs::write("ParseResult.json", &json_output).ok();

    // Also save preprocessing info
    let preprocess_json = serde_json::to_string_pretty(&preprocess_info).unwrap();
    fs::write("PreprocessInfo.json", &preprocess_json).ok();

    println!("\nResult saved to ParseResult.json");
    println!("Preprocessing info saved to PreprocessInfo.json");
    println!(
        "Performance: {:.2} KB/s",
        document_len as f64 / 1024.0 / duration.as_secs_f64()
    );
}
