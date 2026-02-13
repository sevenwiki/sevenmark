# Footnotes

<div v-pre>

Footnotes are defined with `{{{#fn ...}}}` blocks.

## Core Behavior

1. `{{{#fn ...}}}` inserts an inline footnote marker and stores the footnote content.
2. `[fn]` renders the currently collected footnotes at that position and clears the collected list.
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

## Rich Content in Footnotes

Footnote content can contain other SevenMark elements:

```sevenmark
See details{{{#fn
Related docs: [[#url="https://example.com" example.com]]
Code: {{{#code #lang="rust" println!("hi"); }}}
}}}.
```

## Mid-Document Footnote Section

You can place `[fn]` to flush footnotes at a specific location.

```sevenmark
Intro text{{{#fn Intro note }}}.

## Footnotes for This Section
[fn]

Next section text{{{#fn Next section note }}}.
```

## Restrictions

- Nested footnote definitions are not allowed (`{{{#fn ... {{{#fn ...}}} ...}}}`).
- Parameter keys are alphanumeric only (for example `#display`, not `#display_text`).
- `[fn]` is a **footnote-list macro**, not an inline reference marker.

## Notes

- Numbering/indexing follows the order of `{{{#fn}}}` definitions in the document.
- Using `[fn]` multiple times is supported (each call flushes the currently collected footnotes).

</div>
