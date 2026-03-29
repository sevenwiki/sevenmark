# Footnotes

<div v-pre>

Footnotes are defined with `{{{#fn ...}}}` blocks.

## Core Behavior

1. `{{{#fn ...}}}` inserts an inline footnote marker and stores the footnote content.
2. `[fn]` renders the currently collected footnotes at that position and clears the pending list.
3. If `[fn]` is not used, remaining footnotes are rendered automatically at the end of the document.

## Basic Usage

```sevenmark
This sentence has a note{{{#fn First footnote content. }}}.
Another sentence{{{#fn Second footnote content. }}}.

[fn]
```

## Default End-of-Document Rendering

```sevenmark
Paragraph A{{{#fn Footnote A }}}.
Paragraph B{{{#fn Footnote B }}}.

// No [fn] macro here
// -> Footnotes render at the document end
```

## Custom Marker Text (`#display`)

Use `#display` when you want a custom marker label instead of the numeric index.

```sevenmark
Main text{{{#fn #display="*" Custom marker footnote. }}}.
```

## Named Footnotes (`#name`)

Use `#name` to create a reusable named footnote. The first occurrence defines the footnote, and later occurrences with the same name link back to the original entry.

```sevenmark
API limits{{{#fn #name="rate-limit" Requests are limited to 100 per minute. }}}.
See also{{{#fn #name="rate-limit" Later named references point back to the first definition. }}}.

[fn]
```

Notes about named footnotes:

- The rendered marker text uses the `#name` value.
- Later definitions with the same `#name` become references only; their content is not added again.
- Named footnotes stay globally addressable across multiple `[fn]` flushes within the same render pass.

## Rich Content in Footnotes

Footnote content can contain other SevenMark elements:

```sevenmark
See details{{{#fn
Related docs: [[#url="https://example.com" example.com]]
Code: {{{#code #lang="rust"
println!("hi");
}}}
}}}.
```

## Mid-document Footnote Section

You can place `[fn]` to flush footnotes at a specific location.

```sevenmark
Intro text{{{#fn Intro note }}}.

## Footnotes for This Section
[fn]

Next section text{{{#fn Next section note }}}.
```

## Restrictions

- Nested footnote definitions are not allowed (`{{{#fn ... {{{#fn ...}}} ...}}}`).
- `[fn]` is a footnote-list macro, not an inline reference marker.
- Parameter keys may contain letters, digits, underscores, and hyphens.

## Notes

- Numbering follows the order of unnamed `{{{#fn}}}` definitions in the document.
- Using `[fn]` multiple times is supported; each call flushes only the footnotes collected so far.

</div>
