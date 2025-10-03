use sevenmark::sevenmark::transform::{DocumentNamespace, WikiClient, preprocess_sevenmark};
use std::fs;
use std::time::Instant;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    // WikiClient 생성 (로컬 서버 사용)
    let http_client = reqwest::Client::new();
    let base_url =
        std::env::var("WIKI_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let wiki_client = WikiClient::new(http_client, base_url.clone());

    println!("Using wiki server: {}\n", base_url);

    let start_time = Instant::now();
    let result = preprocess_sevenmark(
        DocumentNamespace::Document,
        "string".to_string(),
        &input_content,
        &wiki_client,
    )
    .await;
    let duration = start_time.elapsed();

    match result {
        Ok(processed) => {
            println!(
                "Processed {} elements in {:?}",
                processed.ast.len(),
                duration
            );

            println!("\n=== Processing Info ===");
            println!("Media files: {} found", processed.media.len());
            for media in &processed.media {
                println!("  - {}", media);
            }

            println!("\nCategories: {} found", processed.categories.len());
            for category in &processed.categories {
                println!("  - {}", category);
            }

            if let Some(redirect) = &processed.redirect {
                println!("\nRedirect to: {}", redirect);
            }

            // Save AST
            let json_output = serde_json::to_string_pretty(&processed.ast).unwrap();
            fs::write("ProcessResult.json", &json_output).ok();

            // Save full result
            let full_json = serde_json::to_string_pretty(&processed).unwrap();
            fs::write("ProcessedDocument.json", &full_json).ok();

            println!("\nResults saved:");
            println!("  - ProcessResult.json (AST only)");
            println!("  - ProcessedDocument.json (full result)");
            println!(
                "\nPerformance: {:.2} KB/s",
                document_len as f64 / 1024.0 / duration.as_secs_f64()
            );
        }
        Err(e) => {
            eprintln!("Error processing document: {}", e);
            eprintln!("\nMake sure wiki server is running at: {}", base_url);
            std::process::exit(1);
        }
    }
}
