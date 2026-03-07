# {{PACKAGE_NAME}}

Bundler-target WebAssembly package for the SevenMark parser.

## Install

```bash
pnpm add {{PACKAGE_NAME}}
```

## Usage

```ts
import init, {
  parse_sevenmark,
  parse_sevenmark_to_codemirror,
} from "{{PACKAGE_NAME}}";

await init();

const ast = JSON.parse(parse_sevenmark("== SevenMark =="));
const codemirrorAst = JSON.parse(parse_sevenmark_to_codemirror("== SevenMark =="));
```

This package is built from `crates/sevenmark_wasm` in the main repository:
https://github.com/sevenwiki/sevenmark
