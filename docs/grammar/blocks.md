# Block Elements

<div v-pre>

SevenMark supports various block-level elements.

## Headers

Use `#` symbols to create headers. Levels 1-6 are supported.

```sevenmark
# Level 1 Header
## Level 2 Header
### Level 3 Header
#### Level 4 Header
##### Level 5 Header
###### Level 6 Header
```

### Collapsible Headers

Add `-` after header to make it collapsible:

```sevenmark
## Collapsible Header-
This content will be hidden when header is collapsed.
```

## Horizontal Line

Use three hyphens (`---`) to create a horizontal line:

```sevenmark
---
```

## Blockquotes

Use `{{{#quote}}}` syntax for quoted text:

```sevenmark
{{{#quote
This is a quoted text block.
It can span multiple lines.
}}}
```

### Styled Blockquotes

Parameters can be used to apply styling:

```sevenmark
{{{#quote #style="font-style:italic"
This is a quoted text block with some **bold** content.
}}}
```

## Fold

Create collapsible sections:

```sevenmark
{{{#fold
[[Summary text]]
[[Hidden detailed content]]
}}}
```

### Styled Fold

```sevenmark
{{{#fold #style="border: 1px solid #ccc;"
[[Custom styled fold]]
[[Content]]
}}}
```

## Code Blocks

Display programming code with syntax highlighting:

```sevenmark
{{{#code #lang="rust"
fn main() {
    println!("Hello, world!");
}
}}}
```

### Code Block without Language

```sevenmark
{{{#code
plain text code
}}}
```

## TeX Math

Display mathematical expressions:

### Inline Math

```sevenmark
{{{#tex E = mc^2 }}}
```

### Block Math

```sevenmark
{{{#tex #block
E = mc^2
}}}
```

## Literal

Display markup literally without processing:

```sevenmark
{{{
**This will not be rendered as bold**
*Italic won't work either*
}}}
```

## Styled Elements

Apply custom styling to any content:

```sevenmark
{{{ #style="color:red" #size="16px" Styled text content }}}
```

</div>