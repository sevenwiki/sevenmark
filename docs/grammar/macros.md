# Macros

<div v-pre>

SevenMark macros use square-bracket syntax and render inline dynamic content or utility markers.

## Time and Date

### Current Time

Display the current timestamp:

```sevenmark
Current time: [now]
```

### Current Date

Display the current date:

```sevenmark
Today: [date]
```

### Current Date and Time

Display the current date and time:

```sevenmark
Updated at: [datetime]
```

### Age Calculation

Calculate age from a given date:

```sevenmark
Age calculation: [age(1990-01-15)]
Born in 1995: [age(1995-06-20)]
```

### D-day Counter

Count the number of days to or from an ISO date (`YYYY-MM-DD`):

```sevenmark
Days until launch: [dday(2026-12-31)]
Days since opening day: [dday(2020-01-01)]
```

### Page Count

Render the total page count, optionally scoped to a namespace:

```sevenmark
All pages: [pagecount]
Document pages: [pagecount(Document)]
File pages: [pagecount(File)]
```

## Anchors and Layout

### Named Anchor

Create a named anchor that other links can target:

```sevenmark
[anchor(api-overview)]
## API Overview

[[#document="Guide" #anchor="api-overview" Jump to this section]]
```

### Table of Contents

Render the document table of contents at the current position:

```sevenmark
[toc]
```

`[toc]` uses the document heading tree and links to the rendered section ids. Headings that contain images or embeds do not render those widgets inside the TOC; only textual and inline heading content is reused.

### Line Break

Insert a hard line break:

```sevenmark
Line 1[br]Line 2
```

### Clear Float

Clear preceding floated content and continue from the next block line:

```sevenmark
[[#file="sample.png" #style="float:right; width:180px" Example]]
[clear]
This text starts below the floated image.
```

### Null Macro

The null macro produces no output:

```sevenmark
This text[null]continues without interruption.
```

## Footnote List Placement

### Footnote Flush

`[fn]` renders the currently collected footnotes at that position and clears the pending list:

```sevenmark
Paragraph A{{{#fn First note. }}}.
Paragraph B{{{#fn Second note. }}}.

[fn]
```

If `[fn]` is omitted, any remaining footnotes are rendered automatically at the end of the document.

## Macro Usage in Complex Elements

Macros can be used inside tables, lists, folds, and styled blocks:

```sevenmark
{{{#list #1
[[Release date: [date]]]
[[Countdown: [dday(2026-12-31)]]]
[[Document pages: [pagecount(Document)]]]
[[Jump target: [anchor(changelog)]]]
}}}
```

### In Tables

```sevenmark
{{{#table #caption="Dashboard"
[[#head [[Metric]] [[Value]]]]
[[[[Today]] [[[date]]]]]
[[[[Pages]] [[[pagecount(Document)]]]]]
}}}
```

### In Styled Content

```sevenmark
{{{ #style="color:blue" Last updated: [datetime] }}}
```

## Variables

### Define Variables

Create template variables using `{{{#define}}}`:

```sevenmark
{{{#define #projectName="SevenMark"}}}
{{{#define #version="2.29.0"}}}
{{{#define #author="SevenWiki Team"}}}
```

### Variable Substitution

Reference defined variables using the `[var()]` macro:

```sevenmark
Welcome to [var(projectName)] version [var(version)]!
Created by [var(author)].
```

### Document-order Resolution

Variables are resolved in document order using a single pass:

```sevenmark
{{{#define #baseUrl="https://example.com"}}}
{{{#define #apiUrl="[var(baseUrl)]/api/v1"}}}

API endpoint: [var(apiUrl)]
```

### Conditional Define Pattern

Combine variables and conditionals to build dynamic templates:

```sevenmark
{{{#define #env="production"}}}

{{{#if [var(env)] == "production"
{{{#define #apiHost="https://api.example.com"}}}
}}}

{{{#if [var(env)] == "development"
{{{#define #apiHost="http://localhost:3000"}}}
}}}

Connecting to: [var(apiHost)]
```

## Notes

- Macro dates for `[age(...)]` and `[dday(...)]` use ISO `YYYY-MM-DD` format.
- `[pagecount(namespace)]` passes the namespace string through to the renderer; parser-level namespace validation is not applied.
- `[anchor(name)]` is most useful when paired with media links that use `#anchor`.
- Undefined variables render as an error element in the output.

</div>
