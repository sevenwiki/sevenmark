# API Overview

SevenMark is organized as a Cargo workspace with specialized crates. Each crate handles a distinct stage of the document processing pipeline.

## Crate Structure

| Crate | Purpose | Target |
|-------|---------|--------|
| **sevenmark-parser** | Core parsing engine — text → AST | All platforms |
| **sevenmark-utils** | Shared utilities (UTF-16 offset conversion) | All platforms |
| **sevenmark-wasm** | WebAssembly bindings for browser/Node.js | WASM |
| **sevenmark-transform** | AST preprocessing and postprocessing | Server |
| **sevenmark-server** | REST API server with PostgreSQL | Server |

## Processing Pipeline

```
Input Text
    │
    ▼
┌─────────────────────────────┐
│  1. PARSING                 │  sevenmark-parser
│  - Tokenization (winnow)    │
│  - Recursive descent        │
│  - AST construction         │
│  - Location tracking        │
└─────────────────────────────┘
    │
    ▼
  Raw AST (Vec<Element>)
    │
    ▼
┌─────────────────────────────┐
│  2. PREPROCESSING           │  sevenmark-transform
│  - Resolve [var(...)]       │
│  - Process {{{#include}}}   │
│  - Evaluate {{{#if}}}       │
│  - Collect media references │
│  - Extract categories       │
└─────────────────────────────┘
    │
    ▼
  PreProcessedDocument
    │
    ▼
┌─────────────────────────────┐
│  3. POSTPROCESSING          │  sevenmark-transform
│  - Resolve media URLs       │
│  - Attach file metadata     │
│  - Generate links           │
└─────────────────────────────┘
    │
    ▼
  ProcessedDocument (JSON)
```

## Crate Details

### sevenmark-parser

The core parsing engine. Standalone with minimal dependencies (`winnow`, `serde`). Takes raw text input and produces a structured AST.

- **Input**: `&str` (SevenMark markup text)
- **Output**: `Vec<Element>` (abstract syntax tree)
- **Feature flags**: `include_locations` enables byte offset data in JSON output

See [Parser API](./parser) for usage details.

### sevenmark-utils

Shared utility functions used across crates:

- `convert_ast_to_utf16_offset_json()` — Converts AST with byte offsets to UTF-16 code unit offsets for CodeMirror 6 integration

### sevenmark-wasm

WebAssembly bindings that wrap `sevenmark-parser` for use in browsers and Node.js:

- `parse_sevenmark(input)` — Returns AST JSON with byte offsets
- `parse_sevenmark_to_codemirror(input)` — Returns AST JSON with UTF-16 offsets

Built with `wasm-pack` and exported via `wasm-bindgen`.

### sevenmark-transform

Server-side AST transformations split into two phases:

- **Preprocessor**: Resolves variables, processes includes, evaluates conditionals, collects media references
- **Postprocessor**: Resolves media URLs, attaches file metadata, generates rendered links

Requires database access (via `sea-orm`) for include resolution and media lookup.

### sevenmark-server

Production REST API server built with `axum`:

- Exposes parsing and processing endpoints
- Swagger UI at `/swagger-ui`
- OpenAPI spec at `/api-docs/openapi.json`
- Requires PostgreSQL for document storage and media resolution

## Next Steps

- [Parser API](./parser) — Usage from Rust and JavaScript
- [AST Structure](./ast) — Element types and JSON format
