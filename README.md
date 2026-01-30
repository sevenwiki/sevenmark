# SevenMark

High-performance wiki markup parser for SevenWiki.

[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/discord/1299657351686651935?color=5865F2&logo=discord&logoColor=white&label=Discord)](https://discord.gg/XZ8zy8dngS)

## What's Included

- **Parser**: SIMD-optimized (winnow), 50+ element types, location tracking
- **Transform**: Variable substitution, includes, media resolution
- **Server**: REST API with PostgreSQL, Swagger UI
- **WASM**: Browser/Node.js builds with CodeMirror support

## Crates

| Crate                 | Description                      |
|-----------------------|----------------------------------|
| `sevenmark-parser`    | Core parsing engine (standalone) |
| `sevenmark-utils`     | Shared utilities                 |
| `sevenmark-wasm`      | WebAssembly bindings             |
| `sevenmark-html`      | HTML renderer                    |
| `sevenmark-transform` | AST preprocessing/postprocessing |
| `sevenmark-server`    | REST API server                  |

## Publishing

```bash
cargo xtask publish-dry  # Dry run
cargo xtask publish      # Publish to crates.io
```

## License

Apache-2.0
