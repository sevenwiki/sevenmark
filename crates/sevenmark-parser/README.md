# sevenmark-parser

Core parsing engine for SevenMark wiki markup.

[![Crates.io](https://img.shields.io/crates/v/sevenmark-parser.svg)](https://crates.io/crates/sevenmark-parser)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/sevenwiki/sevenmark/blob/main/LICENSE)

## Features

- SIMD-optimized parsing with winnow
- 50+ AST element types
- Optional location tracking (`include_locations` feature)

## Usage

```rust
use sevenmark_parser::core::parse_document;

let input = "**bold** and *italic*";
let ast = parse_document(input);
```

## License

Apache-2.0
