use sevenmark_formatter::{FormatConfig, format_document};
use sevenmark_parser::core::parse_document;
use std::fs;

fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

fn main() {
    let input = fs::read_to_string("ToParse.sm").expect("ToParse.sm file not found");
    let normalized = normalize_newlines(&input);

    println!("Input ({} bytes):\n{}\n", normalized.len(), "=".repeat(50));

    let ast = parse_document(&normalized);
    let formatted = format_document(&ast, &FormatConfig::default());

    fs::write("FormatResult.sm", &formatted).ok();

    println!("{}", formatted);
    println!("\n{}", "=".repeat(50));
    println!("Result saved to FormatResult.sm");
}
