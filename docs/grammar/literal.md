# Literal Blocks

<div v-pre>

Literal blocks display content exactly as written, without processing any SevenMark syntax. They are useful for showing raw markup or preventing interpretation of special characters.

## Basic Literal Block

Use `{{{` and `}}}` (without any element name) to create a literal block:

```sevenmark
{{{
**This will not be processed as bold**
*This will not be italic*
[[This will not be a link]]
}}}
```

The content is preserved exactly as written, with no SevenMark syntax processing applied.

## Literal vs Code Blocks

### Literal Block

Shows content as-is, without processing SevenMark syntax:

```sevenmark
{{{
This is literal text.
**Not bold** *Not italic*
{{{#code}}} This won't create a code block
}}}
```

### Code Block

Preserves content without processing SevenMark syntax, but marked as code:

```sevenmark
{{{#code
This is code.
**Still not bold** but marked as code
}}}
```

**Key Difference**: Literal blocks preserve raw text, code blocks preserve text but mark it as code.

## Common Use Cases

### Showing SevenMark Syntax

Perfect for documentation and tutorials:

```sevenmark
To create bold text, use this syntax:

{{{
**Bold text here**
}}}

For links, use:

{{{
[[#url="https://example.com" Link text]]
}}}
```

### Raw HTML or XML

```sevenmark
{{{
<div class="container">
  <h1>This HTML won't be parsed</h1>
  <p>It will be displayed as text</p>
</div>
}}}
```

### Configuration Examples

```sevenmark
{{{
# Configuration file
server.port = 8080
server.host = localhost
}}}
```

### Escaping Complex Markup

When you need to show complex nested structures:

```sevenmark
{{{
{{{#table
  [[[[Cell 1]] [[Cell 2]]]]
  [[[[Cell 3]] [[Cell 4]]]]
}}}
}}}
```

## Literal Blocks in Documentation

### Before and After Examples

Show the markup and its rendered result:

**Markup:**
```sevenmark
{{{
{{{#list #1
[[First item]]
[[Second item]]
}}}
}}}
```

**Result:**
```sevenmark
{{{#list #1
[[First item]]
[[Second item]]
}}}
```

### Syntax Reference

```sevenmark
Create a list with this syntax:

{{{
{{{#list #1
[[Item 1]]
[[Item 2]]
}}}
}}}

The #1 parameter creates numbered items.
```

## Whitespace Preservation

Literal blocks preserve all whitespace and line breaks:

```sevenmark
{{{
Line 1
  Indented Line 2
    More indented Line 3

New paragraph after blank line
}}}
```

## Special Characters

No escaping is needed inside literal blocks:

```sevenmark
{{{
Characters that normally need escaping:
* asterisks *
_ underscores _
{ braces }
[ brackets ]
\ backslashes \

All displayed literally!
}}}
```

## Literal in Complex Structures

### Literal in Lists

```sevenmark
{{{#list #1
[[Raw syntax: {{{**bold** *italic*}}}]]
[[Another example: {{{[[link]]}}}]]
}}}
```

### Literal in Tables

```sevenmark
{{{#table
[[[[Syntax]] [[Description]]]]
[[[[{{{**text**}}}]] [[Bold text]]]]
[[[[{{{*text*}}}]] [[Italic text]]]]
}}}
```

### Literal in Folds

```sevenmark
{{{#fold
[[Show Raw Markup]]
[[
{{{
{{{#code #lang="rust"
fn main() {
    println!("Hello!");
}
}}}
}}}
]]
}}}
```

## Nested Braces

Be careful with nested braces - the literal block ends at the first `}}}`:

```sevenmark
{{{
Content here
}}}
This is outside the literal block
```

If you need to show `}}}` inside a literal block, you'll need to use other approaches:

```sevenmark
{{{#code
Use code blocks to show }}} inside
}}}
```

## Comparison with Other Elements

| Element | Purpose | Syntax Processing |
|---------|---------|-------------------|
| **Literal `{{{}}}`** | Preserve raw text as-is | None - completely disabled |
| **Code `{{{#code}}}`** | Preserve text as code | None - syntax disabled |
| **Escape `\`** | Escape single characters | Selective - only escaped chars |
| **Comment `//`** | Hide text completely | N/A - not included in AST |

## Use Cases Summary

### Use Literal Blocks When:
- Showing SevenMark syntax examples
- Displaying raw markup for documentation
- Preventing interpretation of special characters in large blocks
- Creating "before and after" examples
- Showing configuration files or templates

### Use Code Blocks Instead When:
- Working with programming code
- You want to mark content as code
- You want language-specific metadata (e.g., `#lang="rust"`)

### Use Escaping Instead When:
- You only need to escape a few characters
- You want the rest of the text to be formatted normally
- Working with inline content

## Technical Notes

- Literal blocks use `{{{` and `}}}` without any element identifier (no `#` after the braces)
- All SevenMark syntax is ignored inside literal blocks
- Whitespace and line breaks are preserved exactly
- No parameters are supported for literal blocks
- Literal blocks are distinct from comments (which hide content) and code blocks (which mark content as code)

</div>