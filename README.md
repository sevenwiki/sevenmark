# SevenMark

High-performance wiki markup parser for SevenWiki.

[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-AGPL--3.0--only-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/discord/1299657351686651935?color=5865F2&logo=discord&logoColor=white&label=Discord)](https://discord.gg/XZ8zy8dngS)

## What's Included

- **Parser**: SIMD-optimized (winnow), 50+ element types, location tracking
- **Transform**: Variable substitution, includes, media resolution
- **Server**: REST API with PostgreSQL, Swagger UI
- **WASM**: Browser/Node.js builds with CodeMirror support
- **Editor Support**: VS Code extension and JetBrains plugin via LSP

## Crates

| Crate                       | Description                             |
|-----------------------------|-----------------------------------------|
| `sevenmark_ast`             | AST types and traversal utilities       |
| `sevenmark_semantic`        | Semantic analysis helpers               |
| `sevenmark_parser`          | Core parsing engine (standalone)        |
| `sevenmark_utils`           | Shared utilities                        |
| `sevenmark_wasm`            | WebAssembly bindings                    |
| `sevenmark_html`            | HTML renderer                           |
| `sevenmark_transform`       | AST preprocessing/postprocessing        |
| `sevenmark_language_server` | Language Server Protocol implementation |
| `sevenmark_server`          | REST API server                         |

## Editor Support

| Editor    | Path                 | How it works                                           |
|-----------|----------------------|--------------------------------------------------------|
| VS Code   | `editors/vscode/`    | LSP client via `vscode-languageclient`                 |
| JetBrains | `editors/jetbrains/` | Built-in LSP API (`com.intellij.modules.lsp`, 2024.2+) |

Both connect to the same `sevenmark_language_server` binary. Install it via PATH or use the bundled binary in platform-specific release packages.

## Publishing

```bash
cargo xtask publish-dry  # Dry run
cargo xtask publish      # Publish to crates.io
```

## License

AGPL-3.0-only
