# {{PACKAGE_NAME}}

Bundler-target WebAssembly package for the SevenMark LSP.

## Install

```bash
pnpm add {{PACKAGE_NAME}}
```

## Usage

```ts
import init, { handle_lsp_message } from "{{PACKAGE_NAME}}";

await init();

const result = JSON.parse(
  handle_lsp_message(
    JSON.stringify({
      jsonrpc: "2.0",
      id: 1,
      method: "initialize",
      params: {},
    }),
  ),
);
```

This package is built from `crates/sevenmark_wasm_lsp` in the main repository:
https://github.com/sevenwiki/sevenmark
