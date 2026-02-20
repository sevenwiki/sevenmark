use sevenmark_formatter::{FormatConfig, format_document};
use sevenmark_parser::core::parse_document;
use std::fs;

fn main() {
    let input = fs::read_to_string("ToParse.sm").expect("ToParse.sm file not found");

    println!("Input ({} bytes):\n{}\n", input.len(), "=".repeat(50));

    let ast = parse_document(&input);
    let formatted = format_document(&ast, &FormatConfig::default());

    fs::write("FormatResult.sm", &formatted).ok();

    println!("{}", formatted);
    println!("\n{}", "=".repeat(50));
    println!("Result saved to FormatResult.sm");
}
