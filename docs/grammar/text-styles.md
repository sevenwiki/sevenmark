# Text Styles

<div v-pre>

SevenMark supports various text formatting styles.

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

## Combined Formatting

Multiple formatting can be combined:

```sevenmark
**Bold with *italic* inside**
__Underline with **bold** inside__
This text has **bold**, *italic*, __underlined__, ~~strikethrough~~, ^^superscript^^, and ,,subscript,, formatting.
```

## Escaping

Use backslash (`\`) to escape special characters:

```sevenmark
\*Display asterisk literally\*
\**Display bold markers literally\**
```

</div>