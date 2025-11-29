# SevenMark

A high-performance Domain Specific Language (DSL) parser designed for SevenWiki platform.

[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/discord/1299657351686651935?color=5865F2&logo=discord&logoColor=white&label=Discord)](https://discord.gg/XZ8zy8dngS)

## Overview

**SevenMark** is a sophisticated wiki markup parser written in Rust that transforms wiki-style text into structured Abstract Syntax Trees (AST). It combines the power of parser combinators (winnow) with comprehensive wiki features to provide a fast, reliable, and extensible parsing solution.

The project is organized as a **Cargo workspace** with three specialized crates:
- **sevenmark-parser** - Core parsing engine (can be used standalone)
- **sevenmark-transform** - AST preprocessing and postprocessing utilities
- **sevenmark-server** - Production-ready REST API server

### Key Features

- 🚀 **High Performance** - SIMD-optimized parsing with >10 KB/s throughput
- 📊 **50+ Element Types** - Comprehensive markup support for all wiki needs
- 🎯 **Precise Location Tracking** - Every element tracks its source position (byte offsets)
- 🔄 **Multiple Deployment Targets** - Standalone library, REST API Server, or WebAssembly
- 📦 **Modular Workspace** - Three specialized crates for different use cases
- 📍 **JSON Serialization** - Full AST serialization with optional position data
- 🛡️ **Graceful Error Handling** - Unparseable content becomes Error elements
- 🔧 **Extensible Architecture** - Modular parser design for easy additions

## Architecture

### Workspace Structure

SevenMark is organized as a Cargo workspace with three specialized crates:

```
sevenmark/
├── Cargo.toml                 # Workspace root
├── sevenmark-parser/          # Core parsing library
│   ├── src/
│   │   ├── ast/               # 50+ AST element definitions
│   │   │   ├── mod.rs         # SevenMarkElement enum
│   │   │   ├── elements.rs    # Basic element structs
│   │   │   ├── expression.rs  # Expression & IfElement
│   │   │   ├── table.rs       # Table structures
│   │   │   ├── list.rs        # List structures
│   │   │   ├── location.rs    # Location & Parameter types
│   │   │   └── traversable.rs # Traversable trait
│   │   ├── core.rs            # Main parse_document() entry point
│   │   ├── context.rs         # Parsing context & recursion management
│   │   ├── error.rs           # Error types
│   │   └── parser/            # All parsing logic
│   │       ├── element.rs     # Parser router
│   │       ├── brace/         # {{{#...}}} elements
│   │       ├── bracket/       # [[...]] media elements
│   │       ├── markdown/      # Markdown-style syntax
│   │       ├── macro/         # [var()], [age()], etc.
│   │       ├── expr/    # Condition expression parsing
│   │       ├── comment/       # // and /* */ comments
│   │       ├── escape/        # \ escaping
│   │       ├── text/          # Plain text
│   │       ├── parameter/     # Parameter parsing
│   │       └── token/         # Fallback token parsers
│   └── examples/
│       ├── parse.rs           # Simple parser example
│       └── gen_expected.rs    # Test case expected generator
│
├── sevenmark-transform/       # AST transformation library
│   ├── src/
│   │   ├── preprocessor.rs    # Variable substitution, includes
│   │   ├── postprocessor.rs   # Media URL resolution
│   │   ├── processor.rs       # Full pipeline
│   │   ├── position_converter.rs # Byte → line/column
│   │   └── wiki/              # Wiki-specific utilities
│   └── examples/
│       ├── monaco.rs          # Monaco format converter
│       ├── gen_monaco_expected.rs # Monaco test expected generator
│       ├── debug_conversion.rs
│       └── debug_line_spans.rs
│
└── sevenmark-server/          # REST API server
    ├── src/
    │   ├── api/               # API routes
    │   ├── config/            # Configuration
    │   ├── connection/        # Database connection
    │   ├── errors/            # Error handling
    │   └── main.rs            # Server entry point
    └── examples/
        └── process.rs         # Full preprocessing pipeline
```

### Processing Pipeline

```
Input Text
    ↓
┌─────────────────────────────┐
│  1. PARSING (Core)          │
│  - Tokenization             │
│  - Recursive descent        │
│  - AST construction         │
│  - Location tracking        │
└─────────────────────────────┘
    ↓
  AST (byte positions)
    ↓
┌─────────────────────────────┐
│  2. PREPROCESSING           │
│  - Variable substitution    │
│  - Conditional evaluation   │
│  - Include resolution       │
│  - Media collection         │
│  - Category/redirect        │
└─────────────────────────────┘
    ↓
  PreProcessedDocument
    ↓
┌─────────────────────────────┐
│  3. POSTPROCESSING          │
│  - Media URL resolution     │
│  - File URL attachment      │
│  - Link generation          │
└─────────────────────────────┘
    ↓
  ProcessedDocument
```

## Build Options

### Building Individual Crates

```bash
# Core parser library
cargo build -p sevenmark-parser

# Transform library (preprocessing/postprocessing)
cargo build -p sevenmark-transform

# REST API server
cargo build -p sevenmark-server

# Build entire workspace
cargo build --workspace
```

### Parser Features

```bash
# Include location data in JSON output (parser only)
cargo build -p sevenmark-parser --features include_locations
```

### WebAssembly Builds

WASM builds are provided by `sevenmark-transform`, which includes both parsing and Monaco position conversion.

**Important:** Run these commands from the `sevenmark-transform/` directory:

```bash
cd sevenmark-transform

# Browser (web)
wasm-pack build --target web --features wasm --no-default-features

# Bundler (webpack/vite)
wasm-pack build --target bundler --features wasm --no-default-features

# Node.js / VS Code extensions
wasm-pack build --target nodejs --features wasm --no-default-features
```

Or use `--manifest-path` from workspace root:

```bash
wasm-pack build --target bundler --features wasm --no-default-features --manifest-path sevenmark-transform/Cargo.toml
```

**Exported function:** `parse_sevenmark_to_monaco(input: string): string`

**Note:** `--no-default-features` is required to exclude server-only dependencies (sea-orm, tokio, etc.) that are incompatible with WASM.

### Running Examples

```bash
# Simple parser (ToParse.txt → ParseResult.json)
cargo run --example parse -p sevenmark-parser

# Monaco format converter (with line/column positions)
cargo run --example monaco -p sevenmark-transform

# Full processing pipeline (requires database)
cargo run --example process -p sevenmark-server
```

## REST API Server

Start the server:

```bash
cargo run -p sevenmark-server
```

### Endpoints

**Parse Document:**
```bash
POST /v0/parse
Content-Type: application/json

{
  "content": "**bold** text"
}
```

**Response:**
```json
{
  "categories": [],
  "redirect": null,
  "includes": [],
  "ast": [...]
}
```

**API Documentation:**
- Swagger UI: `http://localhost:8080/swagger-ui`
- OpenAPI spec: `http://localhost:8080/api-docs/openapi.json`

### Configuration

Environment variables (`.env`):

```env
# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=sevenmark
DB_USER=postgres
DB_PASSWORD=password
DB_MAX_CONNECTION=10
DB_MIN_CONNECTION=1

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

## Performance

SevenMark is optimized for high performance:

- **SIMD-optimized** parsing operations via winnow
- **Zero-copy** parsing where possible
- **Efficient memory management** with minimal allocations
- **O(log n)** position lookups via binary search
- **Recursion depth limiting** (default: 16 levels)

**Typical performance:** >10 KB/s on modern hardware

## Development

### Running Tests

```bash
# Test entire workspace (recommended)
cargo test --workspace

# Test with location tracking (includes comprehensive parser tests)
cargo test --workspace --features sevenmark-parser/include_locations

# Test specific crate
cargo test -p sevenmark-parser
cargo test -p sevenmark-transform
cargo test -p sevenmark-server
```

### Test Structure

Tests are organized by category in the `tc/` directory:

```
tc/
├── brace/          # {{{#...}}} elements (code, table, list, etc.)
├── bracket/        # [[...]] media elements
├── markdown/       # Headers, formatting, hline
├── macro/          # [var()], [age()], [now()] macros
├── if/             # Conditional expressions
├── fold/           # Fold elements
├── comment/        # Inline and multiline comments
├── escape/         # Escape sequences
├── complex/        # Complex integration tests
└── monaco/         # Monaco position conversion tests
```

Each category contains:
- `input/*.txt` - Test input files
- `expected/*.json` - Expected JSON output

### Regenerating Expected Files

When parser output changes (e.g., AST structure updates), regenerate expected files:

```bash
# Parser expected files (run from sevenmark-parser/)
cd sevenmark-parser
cargo run --example gen_expected --features include_locations

# Monaco expected files (run from sevenmark-transform/)
cd sevenmark-transform
cargo run --example gen_monaco_expected --features sevenmark-parser/include_locations
```

### Adding New Elements

1. Define AST element in `sevenmark-parser/src/ast.rs`
2. Create parser in appropriate module under `sevenmark-parser/src/parser/`:
   - `brace/` for `{{{#...}}}` elements
   - `bracket/` for `[[...]]` elements
   - `markdown/` for markdown-style syntax
   - `macro/` for `[...]` macros
3. Add to `element_parser` router in `sevenmark-parser/src/parser/element.rs`
4. Implement `Traversable` trait if element has children
5. Add tests in `tests/`

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Follow established parser patterns
4. Add comprehensive tests
5. Submit a pull request

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Links

- **Repository:** https://github.com/sevenwiki/sevenmark
- **Discord:** https://discord.gg/XZ8zy8dngS
- **Documentation:** https://sevenwiki.github.io/sevenmark
- **Issue Tracker:** https://github.com/sevenwiki/sevenmark/issues