use sevenmark_parser::core::parse_document;
use sevenmark_server::connection::database_conn::establish_connection;
use sevenmark_server::connection::r2_conn::establish_revision_storage_connection;
use sevenmark_transform::process_sevenmark;
use std::fs;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let input_content = fs::read_to_string("ToParse.sm").expect("ToParse.sm file not found");
    let document_len = input_content.len();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    // Establish database connection
    let db = establish_connection().await;

    // Establish R2 revision storage connection
    let revision_storage = establish_revision_storage_connection()
        .await
        .expect("Failed to connect to R2 revision storage");

    println!("Using database connection\n");

    let start_time = Instant::now();

    // Parse document first
    let ast = parse_document(&input_content);

    let result = process_sevenmark(ast, &db, &revision_storage).await;
    let duration = start_time.elapsed();

    match result {
        Ok(processed) => {
            println!(
                "Processed {} elements in {:?}",
                processed.ast.len(),
                duration
            );

            println!("\n=== Processing Info ===");
            println!("Categories: {} found", processed.categories.len());
            for category in &processed.categories {
                println!("  - {}", category);
            }

            if let Some(redirect) = &processed.redirect {
                println!("\nRedirect to: {:?}:{}", redirect.namespace, redirect.title);
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
            eprintln!("\nMake sure database is accessible");
            std::process::exit(1);
        }
    }
}
