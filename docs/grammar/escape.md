# Escape Characters

<div v-pre>

SevenMark uses backslash (`\`) to escape special characters and prevent them from being interpreted as markup.

## Basic Escaping

Use backslash to display special characters literally:

```sevenmark
Escaped characters: \{ \} \[ \] \* \_ \~ \^ \, \\
```

## Common Escape Sequences

### Markup Characters

```sevenmark
\*Not italic\*
\**Not bold\**
\__Not underlined\__
\~~Not strikethrough\~~
\^^Not superscript\^^
\,,Not subscript\,,
```

### Braces and Brackets

```sevenmark
\{Not a brace element\}
\{\{\{Not a brace block\}\}\}
\[Not a bracket element\]
\[\[Not media\]\]
```

### Backslash Itself

To display a literal backslash, escape it with another backslash:

```sevenmark
This is a backslash: \\
This is a Windows path: C:\\Users\\Name\\Documents
```

## Escaping in Different Contexts

### In Text

```sevenmark
Regular text with \*escaped asterisks\* and \\backslashes.
```

### In Parameters

```sevenmark
{{{ #style="content: '\*'; color: red" Parameter with escaped content }}}
```

### In Code Blocks

Inside code blocks, most escaping is not necessary as content is treated literally:

```sevenmark
{{{#code
function example() {
    // These don't need escaping: * _ ~ ^ ,
    return "Special chars: {} [] *";
}
}}}
```

### In Lists and Tables

```sevenmark
{{{#list #1
[[Item with \*escaped\* characters]]
[[Another item with \[\[brackets\]\]]]
}}}

{{{#table
[[[[Column 1]] [[Column \*2\*]]]]
[[[[Row with \{\{\{braces\}\}\}]] [[Normal cell]]]]
}}}
```

## When Escaping is NOT Needed

### Inside Literal Blocks

```sevenmark
{{{
This content is literal: **bold** *italic* {{{#code}}} [[media]]
No escaping needed here.
}}}
```

### Inside Comments

```sevenmark
// Comments can contain any characters: **bold** {{{elements}}} [[media]]
/* Same for multiline comments:
   **bold** *italic* {{{#list}}} [[links]]
*/
```

## Escape Sequences Reference

| Character | Escaped | Description |
|-----------|---------|-------------|
| `\*` | `*` | Asterisk (prevents bold/italic) |
| `\_` | `_` | Underscore (prevents underline) |
| `\~` | `~` | Tilde (prevents strikethrough) |
| `\^` | `^` | Caret (prevents superscript) |
| `\,` | `,` | Comma (prevents subscript) |
| `\{` | `{` | Left brace |
| `\}` | `}` | Right brace |
| `\[` | `[` | Left bracket |
| `\]` | `]` | Right bracket |
| `\\` | `\` | Backslash itself |

</div>