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
      link: https://github.com/username/sevenmark

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
**Bold text** and *italic text* can be used together.

{{{#table
[[[[Header 1]] [[Header 2]]]]
[[[[Cell 1]] [[Cell 2]]]]
}}}
```

## Key Features

### Text Styling
- `**bold**`, `*italic*`, `__underline__`, `~~strikethrough~~`
- `^superscript^`, `,subscript,`

### Block Elements
- Tables: `{{{#table}}}`
- Lists: `{{{#list}}}`
- Code: `{{{#code}}}`
- Math: `{{{#tex}}}`

### Wiki Features
- Include: `{{{#include}}}`
- Category: `{{{#category}}}`
- Redirect: `{{{#redirect}}}`

</div>