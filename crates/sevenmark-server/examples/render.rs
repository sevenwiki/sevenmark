use sevenmark_html::{RenderConfig, render_document};
use sevenmark_parser::core::parse_document;
use sevenmark_server::connection::database_conn::establish_connection;
use sevenmark_transform::process_sevenmark;
use std::fs;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let input_content = fs::read_to_string("ToParse.txt").expect("ToParse.txt file not found");
    let document_len = input_content.len();

    println!("Input ({} bytes):\n{}\n", document_len, "=".repeat(50));

    // Establish database connection
    let db = establish_connection().await;

    let start_time = Instant::now();

    // Parse
    let ast = parse_document(&input_content);
    println!("Parsed {} elements", ast.len());

    // Process (resolve includes, media, etc.)
    let processed = process_sevenmark(ast, &db)
        .await
        .expect("Failed to process document");
    println!("Processed {} elements", processed.ast.len());

    // Render to HTML
    let config = RenderConfig {
        edit_url: Some("/edit/TestDocument"),
        file_base_url: Some("https://cdn.example.com/"),
        document_base_url: Some("/Document/"),
        category_base_url: Some("/Category/"),
    };
    let html = render_document(&processed.ast, &config);

    let duration = start_time.elapsed();

    // Save
    fs::write("RenderResult.html", &html).ok();
    println!("Rendered {} bytes in {:?}", html.len(), duration);
    println!("\nResult saved to RenderResult.html");
    println!(
        "Performance: {:.2} KB/s",
        document_len as f64 / 1024.0 / duration.as_secs_f64()
    );
}
