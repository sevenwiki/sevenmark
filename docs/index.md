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
    details: Support for bold, italic, underline, strikethrough, superscript, subscript, and styled inline blocks.
  - title: Structured Block Elements
    details: Tables, lists, folds, blockquotes, code blocks, CSS blocks, and ruby annotations with nested content support.
  - title: Dynamic Utility Macros
    details: Built-in macros for time, dates, anchors, page counters, footnotes, variables, and layout helpers.
  - title: Wiki-Specific Features
    details: Includes, categories, redirects, internal links, external embeds, and media-aware document rendering.
  - title: High Performance
    details: Built with Rust and the winnow parser library for fast parsing and precise source tracking.
  - title: Extensible Architecture
    details: Dedicated AST, formatter, HTML renderer, and LSP layers make the syntax easy to evolve.
---

## Quick Start

<div v-pre>

Try SevenMark syntax:

```sevenmark
{{{#define #release="2.29.0"}}}

[anchor(release-notes)]
# SevenMark [var(release)]

Today: [date]
Updated at: [datetime]
Days until launch: [dday(2026-12-31)]

{{{#table #caption="Release overview" #sortable
    [[#head [[Feature]] [[Value]]]]
    [[[[Current release]] [[[var(release)]]]]]
    [[[[Document pages]] [[[pagecount(Document)]]]]]
}}}

See note{{{#fn #name="release-note" This branch adds utility macros, named footnotes, and richer table/media parameters. }}}.
Repeat the same note{{{#fn #name="release-note" Later named references point back to the first definition. }}}.

[[#document="Guide" #anchor="installation" Jump to installation]]

[fn]
```

## Key Features

### Text Styling

- `**bold**`, `*italic*`, `__underline__`, `~~strikethrough~~`
- `^^superscript^^`, `,,subscript,,`
- Headers: `# ## ### #### ##### ######`
- Collapsible headers: `##! Header` (exclamation after `#`)

### Block Elements

- Tables: `{{{#table #caption="..." #sortable ...}}}`
- Lists: `{{{#list #1 [[Item]] }}}`
- Code: raw block opened by `{{{#code` and closed by matching triple-brace depth
- Math: raw block opened by `{{{#tex` and closed by matching triple-brace depth (`#block` optional)
- CSS: raw block opened by `{{{#css` and closed by matching triple-brace depth
- Quotes: `{{{#quote content }}}`
- Folds: `{{{#fold [[summary]] [[details]] }}}`

### Media and Links

- Images: `[[#file="image.png" Alt text]]`
- Links: `[[#url="https://example.com" Link text]]`
- Wiki pages: `[[#document="PageName" Link text]]`
- Fragment links: `[[#document="PageName" #anchor="section-id" Jump]]`
- Theme-aware media: `[[#file="logo.svg" #theme="dark" Dark logo]]`

### Wiki Features

- Include: `{{{#include PageName }}}` or `{{{#include #namespace="Document" PageName }}}`
- Category: `{{{#category Category Name }}}`
- Redirect: `{{{#redirect TargetPage }}}`

### Advanced Features

- Comments: `// inline` and `/* multiline */`
- Macros: `[now]`, `[date]`, `[datetime]`, `[dday(1990-01-01)]`, `[pagecount]`, `[pagecount(Document)]`, `[anchor(name)]`, `[age(1990-01-01)]`, `[var(name)]`, `[br]`, `[clear]`, `[fn]`, `[null]`
- Variables: `{{{#define #key="val"}}}` + `[var(key)]`
- Footnotes: `{{{#fn #display="*" Note}}}` and reusable named footnotes with `{{{#fn #name="api" Note}}}`
- Conditionals: `{{{#if [var(x)] == "value" :: content }}}`
- Operators: `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `||`, `!`, `true`, `false`, `null`
- Styling parameters: `#style`, `#color`, `#size`, `#dark-color`, `#dark-bgcolor`, `#caption`, `#sortable`, `#theme`, `#anchor`, `#x`, `#y`
- Escaping: `\*literal\*`, `\{\{\{not-element\}\}\}`

</div>
