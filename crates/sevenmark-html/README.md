# sevenmark-html

HTML renderer for SevenMark AST.

[![Crates.io](https://img.shields.io/crates/v/sevenmark-html.svg)](https://crates.io/crates/sevenmark-html)
[![License](https://img.shields.io/badge/license-AGPL--3.0--only-blue.svg)](https://github.com/sevenwiki/sevenmark/blob/main/LICENSE)

## Usage

```rust
use sevenmark_parser::core::parse_document;
use sevenmark_html::render_document;

let input = "**bold** and *italic*";
let ast = parse_document(input);
let html = render_document(&ast);
```

## License

AGPL-3.0-only
