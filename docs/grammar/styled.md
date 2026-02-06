# Styled Elements

<div v-pre>

Styled elements allow you to apply custom styling parameters to any content.

## Basic Styled Element

Use `{{{` with parameters but no element identifier:

```sevenmark
{{{ #style="color:red" Red text }}}
{{{ #size="20px" Large text }}}
{{{ #color="blue" #size="16px" Blue text, 16px }}}
```

## Common Parameters

### Color

```sevenmark
{{{ #color="red" Red text }}}
{{{ #color="blue" Blue text }}}
{{{ #color="#00ff00" Green text with hex color }}}
{{{ #color="rgb(255, 0, 255)" Magenta text with RGB }}}
```

### Background Color

```sevenmark
{{{ #bg_color="yellow" Text with yellow background }}}
{{{ #bg_color="#f0f0f0" Text with light gray background }}}
{{{ #color="white" #bg_color="black" White text on black background }}}
```

### Size

```sevenmark
{{{ #size="10px" Small text }}}
{{{ #size="16px" Normal text }}}
{{{ #size="24px" Large text }}}
{{{ #size="2em" Text sized with em units }}}
```

### Opacity

```sevenmark
{{{ #opacity="1.0" Fully opaque (default) }}}
{{{ #opacity="0.7" Slightly transparent }}}
{{{ #opacity="0.5" Half transparent }}}
{{{ #opacity="0.2" Very transparent }}}
```

### Custom Styles

The `#style` parameter accepts any CSS-like styling:

```sevenmark
{{{ #style="font-weight:bold; text-decoration:underline" Bold and underlined }}}
{{{ #style="border: 2px solid red; padding: 10px" Text with border and padding }}}
{{{ #style="transform: rotate(5deg)" Slightly rotated text }}}
```

## Combining Parameters

Multiple parameters can be used together:

```sevenmark
{{{ #color="white" #bg_color="blue" #size="18px" #style="padding: 5px; border-radius: 3px"
Styled text with multiple parameters
}}}
```

## Styled Content with Markup

Styled elements can contain other SevenMark syntax:

```sevenmark
{{{ #color="red"
This text is red and contains **bold** and *italic* formatting.
}}}

{{{ #bg_color="lightyellow" #style="padding: 10px"
Highlighted box with **important** information.
}}}
```

## Common Use Cases

### Highlighting Important Text

```sevenmark
{{{ #bg_color="yellow" #style="padding: 2px 5px"
This is a highlighted warning!
}}}
```

### Creating Colored Labels

```sevenmark
Status: {{{ #color="white" #bg_color="green" #style="padding: 3px 8px; border-radius: 3px" Active }}}

Priority: {{{ #color="white" #bg_color="red" #style="padding: 3px 8px; border-radius: 3px" High }}}
```

### Emphasis Boxes

```sevenmark
{{{ #bg_color="#f0f8ff" #style="border-left: 4px solid blue; padding: 10px; margin: 10px 0"
**Note:** This is an informational box with custom styling.
}}}
```

## Styled in Complex Structures

### In Lists

```sevenmark
{{{#list #1
[[Normal item]]
[[{{{ #color="red" Important item }}}]]
[[{{{ #bg_color="yellow" Highlighted item }}}]]
}}}
```

### In Tables

```sevenmark
{{{#table
[[[[Product]] [[Status]]]]
[[[[Widget A]] [[{{{ #color="green" Available }}}]]]]
[[[[Widget B]] [[{{{ #color="red" Out of Stock }}}]]]]
}}}
```

### In Headers

```sevenmark
# {{{ #color="blue" Styled Header }}}

## {{{ #style="border-bottom: 2px solid red" Underlined Header }}}
```

## Advanced Styling

### Box Styling

```sevenmark
{{{ #style="background: linear-gradient(to right, #f0f0f0, #e0e0e0); border: 1px solid #ccc; border-radius: 8px; padding: 15px; box-shadow: 0 2px 4px rgba(0,0,0,0.1)"
Content in a styled box with gradient background, border, rounded corners, and shadow.
}}}
```

## Nested Styled Elements

```sevenmark
{{{ #bg_color="lightgray" #style="padding: 20px"
Outer styled container

{{{ #bg_color="white" #style="padding: 10px; margin: 10px 0"
Inner styled container with different styling
}}}

Back to outer styling
}}}
```

## Common Style Patterns

### Status Messages

```sevenmark
{{{ #color="white" #bg_color="green" #style="padding: 10px; border-radius: 5px"
✓ Success: Operation completed successfully.
}}}

{{{ #color="white" #bg_color="red" #style="padding: 10px; border-radius: 5px"
✗ Error: Something went wrong.
}}}

{{{ #color="black" #bg_color="yellow" #style="padding: 10px; border-radius: 5px"
⚠ Warning: Please review before proceeding.
}}}
```

### Callout Boxes

```sevenmark
{{{ #bg_color="#e7f3ff" #style="border-left: 4px solid #2196F3; padding: 15px"
**Information:** This is an informational callout box.
}}}

{{{ #bg_color="#fff3e0" #style="border-left: 4px solid #ff9800; padding: 15px"
**Tip:** Here's a helpful tip for users.
}}}
```

### Badges and Tags

```sevenmark
{{{ #color="white" #bg_color="#2196F3" #style="padding: 2px 8px; border-radius: 12px; font-size: 12px"
v2.0
}}}

{{{ #color="white" #bg_color="#4CAF50" #style="padding: 2px 8px; border-radius: 3px; font-size: 11px; text-transform: uppercase"
new
}}}
```

## When to Use Styled Elements

### Use Styled Elements For:
- Custom visual styling needs
- Creating visual emphasis or decoration
- Implementing custom design patterns
- Highlighting specific content

### Use Specific Elements Instead:
- Code → Use `{{{#code}}}` for programming code
- Quotes → Use `{{{#quote}}}` for semantic meaning
- Headers → Use `#`, `##`, etc. for document structure
- Lists → Use `{{{#list}}}` for proper list semantics

## Technical Notes

- Styled elements use `{{{` without an element identifier (e.g., no `#code`, `#list`, etc.)
- **At least one parameter is required** — without any parameter, `{{{ content }}}` is parsed as a [literal block](/grammar/literal) instead of a styled element
- The `#style` parameter accepts styling property-value pairs
- Individual parameters (`#color`, `#size`, etc.) are convenience shortcuts
- Parameters are case-sensitive
- Multiple parameters can be combined
- Parameters are stored in the AST's `CommonStyleAttributes` structure

::: tip Literal vs. Styled
The parser distinguishes between literal blocks and styled elements based on the presence of parameters:

```sevenmark
{{{ This is a literal block (no parameters) }}}
{{{ #color="red" This is a styled element (has parameters) }}}
```

If you intend to style content, always include at least one parameter.
:::

</div>