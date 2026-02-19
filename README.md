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

## Crates

| Crate                 | Description                      |
|-----------------------|----------------------------------|
| `sevenmark_parser`    | Core parsing engine (standalone) |
| `sevenmark_utils`     | Shared utilities                 |
| `sevenmark_wasm`      | WebAssembly bindings             |
| `sevenmark_html`      | HTML renderer                    |
| `sevenmark_transform` | AST preprocessing/postprocessing |
| `sevenmark_server`    | REST API server                  |

## Publishing

```bash
cargo xtask publish-dry  # Dry run
cargo xtask publish      # Publish to crates.io
```

## License

AGPL-3.0-only
