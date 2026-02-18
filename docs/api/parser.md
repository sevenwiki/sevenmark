# Parser API

<div v-pre>

## Rust Usage

### Installation

Add `sevenmark-parser` to your `Cargo.toml`:

```toml
[dependencies]
sevenmark-parser = "2.20"
```

To include byte offset location data in serialized JSON output:

```toml
[dependencies]
sevenmark-parser = { version = "2.20", features = ["include_locations"] }
```

### Basic Parsing

```rust
use sevenmark_parser::core::parse_document;

fn main() {
    let input = "# Hello **World**\n\nThis is *SevenMark*.";
    let elements = parse_document(input);

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&elements).unwrap();
    println!("{}", json);
}
```

### API Reference

#### `parse_document`

```rust
pub fn parse_document(input: &str) -> Vec<Element>
```

The main entry point for parsing SevenMark content.

- **Input**: A string slice containing SevenMark markup
- **Returns**: A vector of `Element` nodes representing the parsed AST
- The parser is forgiving — unparseable content becomes `Element::Error` nodes rather than failing
- Recursion depth is limited to 16 levels by default to prevent stack overflow

### Public Modules

| Module | Description |
|--------|-------------|
| `ast` | AST types (`Element`, `Expression`, `Span`, etc.) |
| `core` | The `parse_document` function |
| `context` | `ParseContext` — parser state management |
| `error` | Error types |
| `parser` | Internal parser combinators (typically not used directly) |

## WebAssembly Usage

### Installation

Build the WASM package from the `sevenmark-wasm` crate:

```bash
cd crates/sevenmark-wasm

# For browsers
wasm-pack build --target web

# For bundlers (webpack, vite)
wasm-pack build --target bundler

# For Node.js / VS Code extensions
wasm-pack build --target nodejs
```

### JavaScript API

#### `parse_sevenmark(input: string): string`

Parse SevenMark text and return AST JSON with **byte offsets**.

```javascript
import init, { parse_sevenmark } from "sevenmark-wasm";

await init();

const result = parse_sevenmark("# Hello **World**");
const ast = JSON.parse(result);
console.log(ast);
```

#### `parse_sevenmark_to_codemirror(input: string): string`

Parse SevenMark text and return AST JSON with **UTF-16 code unit offsets**. Designed for CodeMirror 6 integration where positions must be 0-based UTF-16 offsets.

```javascript
import init, { parse_sevenmark_to_codemirror } from "sevenmark-wasm";

await init();

const result = parse_sevenmark_to_codemirror("# Hello **World**");
const ast = JSON.parse(result);
// Offsets are in UTF-16 code units, compatible with CodeMirror 6
```

### Node.js Usage

```javascript
const { parse_sevenmark, parse_sevenmark_to_codemirror } = require("sevenmark-wasm");

const ast = JSON.parse(parse_sevenmark("**Bold** and *italic*"));
console.log(ast);
```

## Feature Flags

### `include_locations`

When enabled, the `Span` data (start/end byte offsets) is included in serialized JSON output. This is useful for editor integration, source mapping, and error reporting.

**Without** `include_locations` (default):

```json
[
  { "Header": { "level": 1, "children": [{ "Text": { "value": "Hello" } }] } }
]
```

**With** `include_locations`:

```json
[
  {
    "Header": {
      "span": { "start": 0, "end": 7 },
      "level": 1,
      "children": [{ "Text": { "span": { "start": 2, "end": 7 }, "value": "Hello" } }]
    }
  }
]
```

## Error Handling

The parser does not panic or return `Result` — it always produces a `Vec<Element>`. Content that cannot be parsed is wrapped in `Element::Error` nodes:

```rust
let elements = parse_document("malformed {{{ unclosed");
// The unclosed brace block becomes an Error element
// Subsequent valid content is still parsed correctly
```

This "error recovery" approach ensures partial documents remain useful.

</div>
