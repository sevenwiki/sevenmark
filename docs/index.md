---
layout: home

hero:
  name: "SevenMark"
  text: "A DSL for SevenWiki"
  tagline: High-performance Rust-based markup parser
  actions:
    - theme: brand
      text: Get Started
      link: /grammar/text-styles
    - theme: alt
      text: View on GitHub
      link: https://github.com/sevenwiki/sevenmark

features:
  - title: Rich Text Styling
    details: Support for bold, italic, underline, strikethrough, and various text formatting options.
  - title: Structured Block Elements
    details: Complex document structures with tables, lists, folds, blockquotes, and more.
  - title: Wiki-Specific Features
    details: Built-in support for includes, categories, redirects, and other wiki system features.
  - title: High Performance
    details: Built with Rust and winnow parser library for exceptional parsing speed and efficiency.
  - title: Extensible Architecture
    details: Macro system and plugin architecture allow easy extension of functionality.
  - title: Precise Location Tracking
    details: Accurate source position tracking for all parsed elements, enabling debugging and error reporting.
---

## Quick Start

<div v-pre>

Try SevenMark syntax:

```sevenmark
**Bold text** and *italic text* with ^^superscript^^ and ,,subscript,,.

{{{#table
    [[[[Header 1]] [[Header 2]]]]
    [[[[Cell 1]] [[Cell 2]]]]
}}}

{{{#list #1
    [[First item with **bold** text]]
    [[Second item with {{{#code console.log("hello") }}}]]
    [[Third item]]
}}}

Current time: [now] // This is a comment
```

## Key Features

### Text Styling

- `**bold**`, `*italic*`, `__underline__`, `~~strikethrough~~`
- `^^superscript^^`, `,,subscript,,`
- Headers: `# ## ### #### ##### ######`
- Collapsible headers: `## Header-`

### Block Elements

- Tables: `{{{#table [[[[Cell]]]] }}}`
- Lists: `{{{#list #1 [[Item]] }}}`
- Code: `{{{#code #lang="rust" code }}}`
- Math: `{{{#tex formula }}}` or `{{{#tex #block formula }}}`
- Quotes: `{{{#quote content }}}`
- Folds: `{{{#fold [[summary]] [[details]] }}}`

### Media & Links

- Images: `[[#file="image.png" Alt text]]`
- Links: `[[#url="https://example.com" Link text]]`
- Files: `[[#file="doc.pdf" #url="backup" Document]]`

### Wiki Features

- Include: `{{{#include #page="PageName" content }}}`
- Category: `{{{#category Category Name }}}`
- Redirect: `{{{#redirect TargetPage }}}`

### Advanced Features

- Comments: `// inline` and `/* multiline */`
- Macros: `[now]`, `[age(1990-01-01)]`, `[br]`
- Styling: `{{{ #style="css" #color="red" content }}}`
- Parameters: `#style`, `#color`, `#size`, `#x`, `#y`
- Escaping: `\*literal\*`, `\{\{\{not-element\}\}\}`

</div>
