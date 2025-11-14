# SevenMark

A high-performance Domain Specific Language (DSL) parser designed for SevenWiki platform.

[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

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

## Syntax Reference

### Text Formatting

```sevenmark
**bold**
*italic*
__underline__
~~strikethrough~~
^^superscript^^
,,subscript,,
```

**Output styles:** Bold, Italic, Underline, Strikethrough, Superscript, Subscript

### Headers

```sevenmark
# Level 1 Header
## Level 2 Header
### Level 3 Header
#### Level 4 Header
##### Level 5 Header
###### Level 6 Header
###! Folded Header Level 3
```

Headers support 1-6 levels. Add `!` after `#` for collapsible headers.

### Lists

```sevenmark
{{{#list #1
[[First item]]
[[Second item]]
[[Third item]]
}}}

{{{#list #a
[[Item A]]
[[Item B]]
}}}
```

**List Types:**
- `#1` - Numeric (1, 2, 3...)
- `#a` - Lowercase letters (a, b, c...)
- `#A` - Uppercase letters (A, B, C...)
- `#i` - Roman numerals lowercase (i, ii, iii...)
- `#I` - Roman numerals uppercase (I, II, III...)

### Tables

```sevenmark
{{{#table
[[[[Header 1]] [[Header 2]]]]
[[[[Cell 1]] [[Cell 2]]]]
[[[[Cell 3]] [[Cell 4]]]]
}}}
```

**Cell Parameters:**
```sevenmark
{{{#table
[[[[#x="2" Spans 2 columns]]]]
[[[[#y="2" Spans 2 rows]] [[Normal cell]]]]
}}}
```

### Folds (Collapsible Content)

```sevenmark
{{{#fold
[[Summary Title]]
[[Hidden content here with **formatting**]]
}}}

{{{#fold #style="background:#f0f0f0"
[[Styled title]]
[[Content]]
}}}
```

### Code Blocks

```sevenmark
{{{#code #lang="rust"
fn main() {
    println!("Hello, SevenMark!");
}
}}}

{{{#code #lang="python"
def hello():
    print("Hello, SevenMark!")
}}}
```

### Media & Links

```sevenmark
[[#file="image.png" Alt text for image]]
[[#url="https://example.com/image.jpg" External image]]
[[#url="https://rust-lang.org" Official Rust Website]]
[[#file="document.pdf" PDF Document]]
```

**Media Parameters:**
- `#file="filename"` - File reference (resolved via API)
- `#url="https://..."` - Direct URL
- `#document="PageName"` - Wiki page link
- `#category="CategoryName"` - Category link

**Priority:** file > document > category > url

### Variables

**Definition:**
```sevenmark
{{{#define #name="projectName" #value="SevenMark"}}}
{{{#define #name="version" #value="2.0"}}}
```

**Usage:**
```sevenmark
Welcome to [var(projectName)] version [var(version)]!
```

**Important Notes:**
- Variables must be defined before use
- Later definitions override earlier ones (variable shadowing)
- Resolved during preprocessing

### Macros

```sevenmark
[now]                    // Current timestamp
[age(1990-01-15)]        // Calculate age from date (YYYY-MM-DD)
[var(variableName)]      // Variable substitution
[br]                     // Line break
[fn]                     // Footnote reference
[null]                   // Null element
```

### Wiki Elements

**Include (Transclusion):**
```sevenmark
{{{#include
PageName
}}}

{{{#include #namespace="Document"
PageName
}}}
```

**Category:**
```sevenmark
{{{#category
Programming Languages
}}}
```

**Redirect:**
```sevenmark
{{{#redirect
Target Page Name
}}}
```

### Special Elements

**Block Quote:**
```sevenmark
{{{#quote #style="font-style:italic"
This is a quoted text block.
}}}
```

**TeX Math:**
```sevenmark
{{{#tex
E = mc^2
}}}

{{{#tex #block
\sum_{i=1}^{n} x_i
}}}
```

**Ruby Text (Furigana):**
```sevenmark
{{{#ruby #rt="ふりがな"
漢字
}}}
```

**Footnote:**
```sevenmark
{{{#fn
Footnote content here
}}}
```

**Literal Block:**
```sevenmark
{{{ Content with **markup** that is still parsed }}}
```

**Styled Element:**
```sevenmark
{{{ #style="color:red" #size="16px" #bgcolor="yellow"
Styled text content
}}}
```

### Comments

```sevenmark
// Inline comment

/* Multi-line
   comment */
```

### Escaping

```sevenmark
\{ \} \[ \] \* \_ \~ \^ \, \\
```

Use backslash `\` to escape special characters.

### Horizontal Line

```sevenmark
---
```

Use 3-9 dashes at line start.

## Complete Example

```sevenmark
{{{#category
Programming Languages
}}}

{{{#define #name="lang" #value="Rust"}}}
{{{#define #name="year" #value="2015"}}}

# [var(lang)] Programming Language

**[var(lang)]** was released in [var(year)]. It's known for its **memory safety** and __zero-cost abstractions__.

## Key Features

{{{#fold #style="background:#f0f8ff"
[[🔧 **Technical Features** _(click to expand)_]]
[[
{{{#table
[[[[**Feature**]] [[**Description**]]]]
[[[[Memory Safety]] [[No null pointers or buffer overflows]]]]
[[[[Concurrency]] [[Safe parallel programming]]]]
[[[[Performance]] [[Zero-cost abstractions]]]]
}}}
]]
}}}

## Learning Resources

{{{#list #1
[[Official documentation at ,,doc.rust-lang.org,,]]
[[**The Rust Book** - comprehensive guide]]
[[Community forums and Discord]]
}}}

### Example Code

{{{#code #lang="rust"
fn main() {
    println!("Hello, Rust!");
}
}}}

{{{#quote #style="font-style:italic; color:#666"
"Rust is a systems programming language that is fast, memory-safe, and parallel."
— *Mozilla Research Team*
}}}

---

*Page created [age(2015-05-15)] days ago. Last updated: [now]*
```

## Architecture

### Workspace Structure

SevenMark is organized as a Cargo workspace with three specialized crates:

```
sevenmark/
├── Cargo.toml                 # Workspace root
├── sevenmark-parser/          # Core parsing library
│   ├── src/
│   │   ├── ast.rs             # 50+ AST element definitions
│   │   ├── core.rs            # Main parse_document() entry point
│   │   ├── context.rs         # Parsing context & recursion management
│   │   ├── error.rs           # Error types
│   │   └── parser/            # All parsing logic
│   │       ├── element.rs     # Parser router
│   │       ├── brace/         # {{{#...}}} elements
│   │       ├── bracket/       # [[...]] media elements
│   │       ├── markdown/      # Markdown-style syntax
│   │       ├── macro/         # [var()], [age()], etc.
│   │       ├── comment/       # // and /* */ comments
│   │       ├── escape/        # \ escaping
│   │       ├── text/          # Plain text
│   │       ├── parameter/     # Parameter parsing
│   │       └── token/         # Fallback token parsers
│   └── examples/
│       └── parse.rs           # Simple parser example
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

## Dependencies

### sevenmark-parser (Core Parser)

- **winnow** (0.7.13) - High-performance parser combinators with SIMD
- **serde** (1.0.228) - Serialization framework
- **serde_json** (1.0.145) - JSON support
- **line-span** (0.1.5) - Line position calculation
- **paste** (1.0.15) - Macro utilities

**Optional (WASM):**
- **wasm-bindgen** (0.2.105) - Rust ↔ JavaScript bridge
- **js-sys** (0.3.82) - JavaScript standard library
- **web-sys** (0.3.82) - Web API bindings

### sevenmark-transform (Preprocessing/Postprocessing)

- **sevenmark-parser** - Core parser (workspace)
- **sea-orm** (2.0.0-rc.18) - ORM for PostgreSQL
- **anyhow** (1.0.100) - Error handling
- **uuid** (1.18.1) - UUID generation
- **utoipa** (5.4.0) - OpenAPI types
- **tracing** (0.1.41) - Logging

### sevenmark-server (REST API)

- **sevenmark-parser** - Core parser (workspace)
- **sevenmark-transform** - Transform utilities (workspace)
- **axum** (0.8.6) - Web framework
- **tokio** (1.48.0) - Async runtime
- **sea-orm** (2.0.0-rc.18) - ORM for PostgreSQL
- **utoipa** (5.4.0) - OpenAPI documentation
- **utoipa-swagger-ui** (9.0.2) - Swagger UI
- **dotenvy** (0.15.7) - Environment variables
- **validator** (0.20.0) - Input validation
- **tracing** (0.1.41) - Logging

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
# Test all crates
cargo test --workspace --all-features

# Test specific crate
cargo test -p sevenmark-parser
cargo test -p sevenmark-transform
cargo test -p sevenmark-server

# Test with location tracking enabled
cargo test -p sevenmark-parser --features include_locations
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
- **Documentation:** [Coming soon]
- **Issue Tracker:** https://github.com/sevenwiki/sevenmark/issues