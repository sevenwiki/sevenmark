# sevenmark-formatter

Source code formatter for SevenMark AST.

[![Crates.io](https://img.shields.io/crates/v/sevenmark-formatter.svg)](https://crates.io/crates/sevenmark-formatter)
[![License](https://img.shields.io/badge/license-AGPL--3.0--only-blue.svg)](https://github.com/sevenwiki/sevenmark/blob/main/LICENSE)

## Usage

```rust
use sevenmark_parser::core::parse_document;
use sevenmark_formatter::{format_document, FormatConfig};

let input = "**bold** and *italic*";
let ast = parse_document(input);
let output = format_document(&ast, &FormatConfig::default());
```

## License

AGPL-3.0-only