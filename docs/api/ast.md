# AST Structure

The SevenMark parser produces an Abstract Syntax Tree (AST) composed of `Element` nodes. This page documents the element types, expression system, and supporting types.

## Element Enum

Every parsed node is a variant of the `Element` enum. Elements are grouped by category:

### Basic Text

| Variant | Description |
|---------|-------------|
| `Text` | Plain text content |
| `Comment` | Comment block (not rendered) |
| `Escape` | Escaped character (`\*`, `\[`, etc.) |
| `Error` | Unparseable content (error recovery) |

### Text Styles

| Variant | Description |
|---------|-------------|
| `Bold` | `**text**` |
| `Italic` | `*text*` |
| `Underline` | `__text__` |
| `Strikethrough` | `~~text~~` |
| `Superscript` | `^^text^^` |
| `Subscript` | `,,text,,` |

All text style variants share the same `TextStyleElement` structure with `span`, `parameters`, and `children` fields.

### Block Elements

| Variant | Description |
|---------|-------------|
| `Literal` | Raw content block `{{{ text }}}` |
| `Styled` | Styled block `{{{ #param text }}}` |
| `Table` | Table structure `{{{#table ...}}}` |
| `List` | Ordered/unordered list `{{{#list ...}}}` |
| `Fold` | Collapsible section `{{{#fold ...}}}` |
| `BlockQuote` | Quote block `{{{#quote ...}}}` |
| `Code` | Code block `{{{#code ...}}}` |
| `TeX` | Math expression `{{{#tex ...}}}` |
| `Ruby` | Ruby annotation `{{{#ruby ...}}}` |
| `Footnote` | Footnote definition `{{{#footnote ...}}}` |

### Line Elements

| Variant | Description |
|---------|-------------|
| `Header` | Heading (`#`, `##`, ..., `######`) |
| `HLine` | Horizontal rule (`---`) |
| `SoftBreak` | Single newline (paragraph continues) |
| `HardBreak` | Double newline (paragraph break) |

### Wiki Elements

| Variant | Description |
|---------|-------------|
| `Include` | Include another document `{{{#include ...}}}` |
| `Category` | Category assignment `{{{#category ...}}}` |
| `Redirect` | Page redirect `{{{#redirect ...}}}` |
| `Define` | Variable definition `{{{#define ...}}}` |
| `If` | Conditional block `{{{#if ...}}}` |

### Media Elements

| Variant | Description |
|---------|-------------|
| `Media` | Internal media `[[file:...]]` |
| `ExternalMedia` | External embed `[[#youtube ...]]`, `[[#spotify ...]]`, etc. |

### Macros

| Variant | Description |
|---------|-------------|
| `Variable` | Variable reference `[var(name)]` |
| `Mention` | User/discussion mention `<@uuid>` or `<#uuid>` |
| `TimeNow` | Current time `[now]` |
| `Age` | Age calculation `[age(date)]` |
| `Null` | No-op macro `[null]` |
| `FootnoteRef` | Footnote reference `[footnote(id)]` |

## Common Element Fields

Most elements share these fields:

```rust
struct SomeElement {
    pub span: Span,           // byte offsets (start, end)
    pub parameters: Parameters, // #key="value" pairs
    pub children: Vec<Element>, // nested elements
}
```

### Span

```rust
pub struct Span {
    pub start: usize,  // start byte offset in source
    pub end: usize,    // end byte offset in source
}
```

Every element records its position in the source text via `Span`. When serialized with the `include_locations` feature, these appear in JSON output.

### Parameters

```rust
pub struct Parameter {
    pub span: Span,
    pub key: String,
    pub value: Vec<Element>,  // parsed value (may contain macros)
}

pub type Parameters = BTreeMap<String, Parameter>;
```

Parameters are stored as a `BTreeMap` keyed by parameter name. The value is a parsed AST (not just a raw string), allowing macros like `[var(name)]` inside parameter values.

## Expression Enum (Conditionals)

The `{{{#if}}}` element uses an expression tree for its condition:

| Variant | Description |
|---------|-------------|
| `Or` | Logical OR (`\|\|`) |
| `And` | Logical AND (`&&`) |
| `Not` | Logical NOT (`!`) |
| `Comparison` | Comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`) |
| `FunctionCall` | Type conversion (`int()`, `len()`, `str()`) |
| `StringLiteral` | Quoted string value |
| `NumberLiteral` | Integer value |
| `BoolLiteral` | `true` or `false` |
| `Null` | `null` literal |
| `Group` | Parenthesized sub-expression |
| `Element` | Embedded AST element (e.g., `[var(name)]`) |

### Operators

**Comparison operators:**

| Kind | Syntax |
|------|--------|
| `Equal` | `==` |
| `NotEqual` | `!=` |
| `GreaterThan` | `>` |
| `LessThan` | `<` |
| `GreaterEqual` | `>=` |
| `LessEqual` | `<=` |

**Logical operators:**

| Kind | Syntax |
|------|--------|
| `Or` | `\|\|` |
| `And` | `&&` |
| `Not` | `!` |

## Table Types

Tables have a multi-level type hierarchy to support conditional rows and cells:

```
TableElement
  └── children: Vec<TableRowItem>
        ├── Row(TableRowElement)
        │     └── children: Vec<TableCellItem>
        │           ├── Cell(TableCellElement)
        │           └── Conditional(ConditionalTableCells)
        └── Conditional(ConditionalTableRows)
```

### TableElement

```rust
pub struct TableElement {
    pub span: Span,
    pub parameters: Parameters,
    pub children: Vec<TableRowItem>,
}
```

### TableRowItem

```rust
pub enum TableRowItem {
    Row(TableRowElement),
    Conditional(ConditionalTableRows),
}
```

### TableCellItem

```rust
pub enum TableCellItem {
    Cell(TableCellElement),
    Conditional(ConditionalTableCells),
}
```

## List Types

Lists follow a similar pattern to tables for conditional support:

```
ListElement
  └── children: Vec<ListContentItem>
        ├── Item(ListItemElement)
        └── Conditional(ConditionalListItems)
```

### ListElement

```rust
pub struct ListElement {
    pub span: Span,
    pub kind: String,           // list type ("1", "a", "A", "i", "I")
    pub parameters: Parameters,
    pub children: Vec<ListContentItem>,
}
```

### ListContentItem

```rust
pub enum ListContentItem {
    Item(ListItemElement),
    Conditional(ConditionalListItems),
}
```

## Mention Types

```rust
pub enum MentionType {
    Discussion,  // <#uuid>
    User,        // <@uuid>
}
```

## JSON Serialization Example

Given this input:

```sevenmark
# Hello **World**

A paragraph with *italic* text.
```

The serialized AST (simplified) looks like:

```json
[
  {
    "Header": {
      "level": 1,
      "children": [
        { "Text": { "value": "Hello " } },
        { "Bold": { "children": [{ "Text": { "value": "World" } }] } }
      ]
    }
  },
  { "HardBreak": {} },
  {
    "Text": { "value": "A paragraph with " }
  },
  {
    "Italic": {
      "children": [{ "Text": { "value": "italic" } }]
    }
  },
  {
    "Text": { "value": " text." }
  }
]
```

Each element is serialized as an object with the variant name as key. The `Span` fields only appear when the `include_locations` feature is enabled.
