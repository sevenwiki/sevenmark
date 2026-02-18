# Horizontal Line

<div v-pre>

Horizontal lines (also called horizontal rules or dividers) create visual separation between sections of content.

## Basic Horizontal Line

Use three to nine hyphens on a line by themselves:

```sevenmark
---
```

```sevenmark
---------
```

## Valid Syntax

The parser accepts 3 to 9 consecutive hyphens:

```sevenmark
---       // 3 hyphens (minimum)
----      // 4 hyphens
-----     // 5 hyphens
------    // 6 hyphens
-------   // 7 hyphens
--------  // 8 hyphens
--------- // 9 hyphens (maximum)
```

## Usage Examples

### Section Separator

```sevenmark
# Chapter 1

Content of chapter 1...

---

# Chapter 2

Content of chapter 2...
```

### Thematic Break

```sevenmark
Main article content here.

Multiple paragraphs of text.

---

Footer or related information below.
```

### Content Grouping

```sevenmark
## Features

- Feature 1
- Feature 2
- Feature 3

---

## Installation

Step-by-step installation guide...
```

## Horizontal Lines in Documents

### Breaking Up Long Content

```sevenmark
# Documentation

## Introduction

Long introduction text...

---

## Getting Started

Tutorial content...

---

## Advanced Topics

Advanced material...
```

### Separating List Groups

```sevenmark
{{{#list #1
[[Task group 1 item 1]]
[[Task group 1 item 2]]
}}}

---

{{{#list #1
[[Task group 2 item 1]]
[[Task group 2 item 2]]
}}}
```

### Visual Dividers in Tables

```sevenmark
{{{#table
[[[[Section 1]] [[Data]]]]
[[[[Item A]] [[Value 1]]]]
}}}

---

{{{#table
[[[[Section 2]] [[Data]]]]
[[[[Item B]] [[Value 2]]]]
}}}
```

## Important Notes

### Must Be On Own Line

Horizontal lines must appear on their own line:

```sevenmark
<!-- Valid -->
Text above
---
Text below

<!-- Invalid -->
Text---More text
```

### Hyphen Count

- **Minimum**: 3 hyphens (`---`)
- **Maximum**: 9 hyphens (`---------`)
- **Outside range**: Not recognized as horizontal line

```sevenmark
--        // Too few - not a horizontal line
---       // Valid
---------  // Valid
---------- // Too many - not a horizontal line
```

### Line Ending Required

The horizontal line must be followed by a line ending or end of file:

```sevenmark
---
```

or

```sevenmark
---
[end of file]
```

## Styling

Horizontal lines do not support styling parameters. They are semantic elements rendered according to the output format's default styling.

## Use Cases

### Document Structure

 **Good uses:**
- Separating major sections
- Creating visual breaks in long content
- Dividing thematically different content
- Ending sections before new topics

L **Avoid:**
- Overusing (too many dividers clutter the page)
- Between closely related paragraphs
- As decorative elements (use styled elements instead)
- Within lists or tables (use proper structure instead)

### Best Practices

```sevenmark
<!-- Good: Clear section separation -->
# Introduction
Content...

---

# Main Content
More content...

<!-- Less ideal: Too many dividers -->
Paragraph 1
---
Paragraph 2
---
Paragraph 3
---
```

## Comparison with Other Dividers

| Element | Purpose | Syntax |
|---------|---------|--------|
| **Horizontal Line** | Semantic section break | `---` |
| **Styled Elements** | Custom visual divider | `{{{ #style="..." }}}` |
| **Headers** | Section titles | `#`, `##`, etc. |
| **Whitespace** | Soft separation | Empty lines |

### When to Use Each

- **Horizontal Line**: For semantic breaks between content sections
- **Styled Elements**: When you need custom styling or decorative dividers
- **Headers**: When the new section needs a title
- **Whitespace**: For subtle separation within related content

## Technical Notes

- Horizontal lines use Markdown-style syntax: `---`
- Parser accepts 3-9 consecutive hyphens
- Must be on their own line with a line ending
- No parameters or styling options supported
- Semantically represents a thematic break in content
- Implementation: `src/sevenmark/parser/markdown/markdown_hline.rs`

</div>