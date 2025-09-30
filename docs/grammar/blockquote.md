# Blockquotes

<div v-pre>

Blockquotes are used to display quoted text or highlighted content, typically rendered with special formatting like indentation or background color.

## Basic Blockquote

Use `{{{#quote}}}` syntax for quoted text:

```sevenmark
{{{#quote
This is a quoted text block.
It can span multiple lines.
}}}
```

## Styled Blockquotes

Apply styling to blockquotes using parameters:

```sevenmark
{{{#quote #style="font-style:italic"
This is an italicized quoted text block.
}}}

{{{#quote #style="border-left: 4px solid #ccc; padding-left: 10px"
A quote with a left border.
}}}
```

## Blockquotes with Formatting

Blockquotes can contain other SevenMark elements:

```sevenmark
{{{#quote
This quote contains **bold text**, *italic text*, and even `code`.

It can also span multiple paragraphs and include other formatting.
}}}
```

## Common Use Cases

### Citations

```sevenmark
{{{#quote
"The best way to predict the future is to invent it."
 Alan Kay
}}}
```

### Important Notes

```sevenmark
{{{#quote #style="background: #fff3cd; padding: 10px; border-radius: 5px"
**Important:** This feature is deprecated and will be removed in version 3.0.
Please migrate to the new API.
}}}
```

### Nested Content

```sevenmark
{{{#quote
Quoted text with a list:

{{{#list #1
[[First point in the quote]]
[[Second point in the quote]]
}}}

And a code example:

{{{#code #lang="rust"
fn example() {
    println!("Code inside a quote");
}
}}}
}}}
```

## Styling Options

All standard parameters are supported:

- `#style` - Custom CSS styling
- `#color` - Text color
- `#bg_color` - Background color
- `#size` - Font size
- `#opacity` - Opacity level

Example:

```sevenmark
{{{#quote #color="blue" #bg_color="#f0f8ff" #style="padding:15px"
A styled blockquote with custom colors and padding.
}}}
```

## Technical Notes

- Blockquotes are block-level elements
- They can contain any other SevenMark elements
- The `{{{#quote}}}` container wraps all content until `}}}`
- Whitespace and line breaks are preserved within the blockquote
- Parameters are optional; basic quotes work without any parameters

</div>