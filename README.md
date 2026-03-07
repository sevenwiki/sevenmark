# SevenMark

High-performance wiki markup parser for SevenWiki.

[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-AGPL--3.0--only-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/discord/1299657351686651935?color=5865F2&logo=discord&logoColor=white&label=Discord)](https://discord.gg/XZ8zy8dngS)

## What's Included

- **Parser**: SIMD-optimized (winnow), 50+ element types, location tracking
- **Transform**: Variable substitution, includes, media resolution
- **Server**: REST API with PostgreSQL, Swagger UI
- **WASM**: Split parser and LSP packages for browser/Node.js
- **Editor Support**: VS Code extension and JetBrains plugin via LSP

## Crates

| Crate                       | Description                             |
|-----------------------------|-----------------------------------------|
| `sevenmark_ast`             | AST types and traversal utilities       |
| `sevenmark_semantic`        | Semantic analysis helpers               |
| `sevenmark_parser`          | Core parsing engine (standalone)        |
| `sevenmark_utils`           | Shared utilities                        |
| `sevenmark_wasm`            | WebAssembly parser bindings             |
| `sevenmark_wasm_lsp`        | WebAssembly LSP bindings                |
| `sevenmark_html`            | HTML renderer                           |
| `sevenmark_transform`       | AST preprocessing/postprocessing        |
| `sevenmark_lsp_core`        | Transport-agnostic LSP logic            |
| `sevenmark_language_server` | Native LSP server (stdio transport)     |
| `sevenmark_server`          | REST API server                         |

## Editor Support

| Editor    | Path                 | How it works                                           |
|-----------|----------------------|--------------------------------------------------------|
| VS Code   | `editors/vscode/`    | LSP client via `vscode-languageclient`                 |
| JetBrains | `editors/jetbrains/` | Built-in LSP API (`com.intellij.modules.lsp`, 2024.2+) |

Both connect to the same `sevenmark_language_server` binary. Install it via PATH or use the bundled binary in platform-specific release packages.

### Web Editor (WASM)

`sevenmark_wasm_lsp` exports `handle_lsp_message(json)` for running the full LSP in a browser Web Worker - no server proxy needed. Diagnostics, semantic tokens, completions, hover, go-to-definition, folding, and document symbols all work locally via JSON-RPC.

## Publishing

```bash
cargo xtask publish-dry  # Dry run
cargo xtask publish      # Publish to crates.io
```

### npm Package

The WASM crates can also be published as bundler-target npm packages for frontend apps.

```bash
cargo xtask wasm-npm-pack --crate sevenmark_wasm --scope your-scope
cargo xtask wasm-npm-publish --crate sevenmark_wasm --scope your-scope

cargo xtask wasm-npm-pack --crate sevenmark_wasm_lsp --scope your-scope
cargo xtask wasm-npm-publish --crate sevenmark_wasm_lsp --scope your-scope
```

By default this publishes:

- `sevenmark_wasm` as `@your-scope/sevenmark`
- `sevenmark_wasm_lsp` as `@your-scope/sevenmark-lsp`

#### Manual npm Release

Log in to npm first:

```bash
npm login
npm whoami
```

Recommended release sequence:

```bash
cargo xtask wasm-npm-publish --crate sevenmark_wasm --scope sevenwiki --dry-run
cargo xtask wasm-npm-publish --crate sevenmark_wasm_lsp --scope sevenwiki --dry-run

cargo xtask wasm-npm-publish --crate sevenmark_wasm --scope sevenwiki
cargo xtask wasm-npm-publish --crate sevenmark_wasm_lsp --scope sevenwiki
```

This publishes:

- `@sevenwiki/sevenmark`
- `@sevenwiki/sevenmark-lsp`

## License

AGPL-3.0-only
