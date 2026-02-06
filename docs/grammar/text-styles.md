# Text Styles

<div v-pre>

SevenMark supports various text formatting styles using inline markers.

## Basic Formatting

### Bold
Use two asterisks (`**`) to create bold text.

```sevenmark
**Bold text**
```

### Italic
Use single asterisk (`*`) to create italic text.

```sevenmark
*Italic text*
```

### Underline
Use two underscores (`__`) to create underlined text.

```sevenmark
__Underlined text__
```

### Strikethrough
Use two tildes (`~~`) to create strikethrough text.

```sevenmark
~~Strikethrough text~~
```

### Superscript
Use double carets (`^^`) to create superscript text.

```sevenmark
x^^2^^ + y^^2^^ = z^^2^^
```

### Subscript
Use double commas (`,,`) to create subscript text.

```sevenmark
H,,2,,O
```

## Summary Table

| Style | Syntax | Example |
|-------|--------|---------|
| Bold | `**text**` | **Bold** |
| Italic | `*text*` | *Italic* |
| Underline | `__text__` | Underlined |
| Strikethrough | `~~text~~` | ~~Strikethrough~~ |
| Superscript | `^^text^^` | x^^2^^ |
| Subscript | `,,text,,` | H,,2,,O |

## Combined Formatting

Multiple formatting markers can be nested to combine styles:

```sevenmark
**Bold with *italic* inside**
*Italic with **bold** inside*
__Underline with **bold** inside__
~~Strikethrough with *italic* inside~~
**__Bold and underlined__**
```

### Deep Nesting

Styles can be nested multiple levels deep:

```sevenmark
**Bold with __underline and *italic*__ together**
This text has **bold**, *italic*, __underlined__, ~~strikethrough~~, ^^superscript^^, and ,,subscript,, formatting.
```

## Text Styles Inside Other Elements

### In Headers

```sevenmark
# **Bold** Header with *italic*

## ~~Deprecated~~ Section
```

### In Lists

```sevenmark
{{{#list #1
[[**Important** item]]
[[Item with *emphasis* and __underline__]]
[[~~Removed~~ item (kept for reference)]]
}}}
```

### In Tables

```sevenmark
{{{#table
[[[[**Header 1**]] [[*Header 2*]] [[__Header 3__]]]]
[[[[Normal cell]] [[**Bold cell**]] [[~~Struck cell~~]]]]
}}}
```

### In Styled Elements

```sevenmark
{{{ #color="red" **Bold red text** with *italic* }}}
```

### In Blockquotes

```sevenmark
{{{#quote
**Important:** This is a *critical* note with __emphasis__.
}}}
```

## Edge Cases

### Empty Markers

Markers with no content between them are treated as literal text:

```sevenmark
****
```

### Adjacent Styles

Different styles can appear next to each other without spaces:

```sevenmark
**bold***italic*__underline__
```

### Markers Inside Words

Style markers work at word boundaries:

```sevenmark
un**frigging**believable
```

### Unmatched Markers

An opening marker without a closing marker is treated as literal text:

```sevenmark
This has an unmatched ** asterisk
```

## Escaping

Use backslash (`\`) to prevent markers from being interpreted as formatting:

```sevenmark
\*Not italic\*
\**Not bold\**
\__Not underlined\__
\~~Not strikethrough\~~
\^^Not superscript\^^
\,,Not subscript\,,
```

Escaping is useful when you need to display the literal marker characters in your text.

</div>
