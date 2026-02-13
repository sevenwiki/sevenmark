# Parameters

<div v-pre>

SevenMark uses a parameter system to pass key-value data to elements.

## How Parameters Work

Parameters are **generic key-value pairs** — the parser accepts `#key="value"` and stores it in AST without semantic validation. The key format itself is constrained to ASCII alphanumeric characters (`[A-Za-z0-9]+`), and meaning is decided by the **renderer** (or postprocessor).

This means:
- `#color`, `#style`, `#lang` are not special to the parser — they are conventions used by renderers
- You can pass any **alphanumeric** parameter name and the parser will accept it
- Unknown parameters are silently ignored by the renderer (they don't cause errors)

## Parameter Syntax

Parameters use `#key="value"` for key-value pairs, or `#flag` for boolean flags (presence means enabled):

```sevenmark
{{{ #style="color:red" #size="16px" Red text }}}
{{{#list #1 #style="margin-left:20px"
[[Item 1]]
[[Item 2]]
}}}
```

### Quoted Values

Parameter values are enclosed in double quotes. Quotes inside values can be escaped:

```sevenmark
{{{ #style="font-family: \"Arial\", sans-serif" Text }}}
```

### Boolean Flags

Some parameters act as flags — their presence alone enables a feature:

```sevenmark
{{{#tex #block
\sum_{i=1}^{n} x_i
}}}

[[#youtube #id="dQw4w9WgXcQ" #autoplay #mute]]
```

## Common Renderer Conventions

The following parameters are commonly recognized by SevenMark renderers. Remember, these are **renderer conventions**, not parser-level features.

### Styling Parameters

| Parameter | Description | Used By |
|-----------|-------------|---------|
| `#style` | Inline CSS styles | Styled elements, tables, lists |
| `#color` | Text color | Styled elements |
| `#bgcolor` | Background color | Styled elements |
| `#size` | Font size | Styled elements |
| `#opacity` | Opacity level | Styled elements |

## Element-Specific Parameters

### Table Parameters

For table cells, use `#x` for colspan and `#y` for rowspan:

```sevenmark
{{{#table
[[[[#x="2" Spans 2 columns]] [[Normal cell]]]]
[[[[#y="2" Spans 2 rows]] [[Cell 1]] [[Cell 2]]]]
[[[[Cell 3]] [[Cell 4]]]]
}}}
```

### List Parameters

Specify list type:

```sevenmark
{{{#list #1      // Numeric (1, 2, 3...)
{{{#list #a      // Lowercase (a, b, c...)
{{{#list #A      // Uppercase (A, B, C...)
{{{#list #i      // Roman lowercase (i, ii, iii...)
{{{#list #I      // Roman uppercase (I, II, III...)
```

### Code Parameters

Specify programming language for syntax highlighting:

```sevenmark
{{{#code #lang="rust"
fn main() {
    println!("Hello, world!");
}
}}}
```

### TeX Parameters

Use `#block` for block-level math display:

```sevenmark
{{{#tex #block
\sum_{i=1}^{n} x_i = x_1 + x_2 + \cdots + x_n
}}}
```

## Parameter Combinations

Multiple parameters can be combined:

```sevenmark
{{{ #style="border: 2px solid blue;" #color="red" #size="18px" 
Multi-styled text with border, color, and size
}}}

{{{#list #1 #style="color:green; margin-left:30px"
[[Green numbered list]]
[[With custom margin]]
}}}
```

## Nested Parameter Usage

Parameters work in nested structures:

```sevenmark
{{{#table #style="border-collapse:collapse"
[[[[**Header 1**]] [[*Header 2*]] [[~~Header 3~~]]]]
[[[[Simple cell]] [[{{{#code #lang="python" print("nested code") }}}]] [[List: {{{#list #a [[Item A]] [[Item B]] }}}]]]]
}}}
```

</div>
