# Parameters

<div v-pre>

SevenMark uses a parameter system to pass key-value data to elements.

## How Parameters Work

Parameters are generic key-value pairs. The parser accepts `#key="value"` and stores the parsed value in the AST without semantic validation. Parameter names support Unicode letters, digits, underscore, and hyphen (`[\p{L}\p{N}_-]+`).

This means:

- `#color`, `#style`, `#lang`, `#caption`, and `#theme` are renderer conventions, not parser keywords
- Hyphenated keys such as `#dark-color` and `#dark-bgcolor` are valid
- Unknown parameters are silently ignored by renderers that do not use them

## Parameter Syntax

Parameters use `#key="value"` for key-value pairs, or `#flag` for boolean flags where presence means enabled:

```sevenmark
{{{ #style="color:red" #size="16px" Red text }}}

{{{#table #caption="Inventory" #sortable
[[#head [[Product]] [[Price]]]]
[[[[Laptop]] [[$1200]]]]
}}}
```

### Quoted Values

Parameter values use double quotes. Quotes inside values can be escaped:

```sevenmark
{{{ #style="font-family: \"Arial\", sans-serif" Text }}}
```

### Boolean Flags

Some parameters act as flags:

```sevenmark
{{{#tex #block
\sum_{i=1}^{n} x_i
}}}

{{{#table #sortable
[[#head [[Name]] [[Value]]]]
[[[[A]] [[1]]]]
}}}
```

## Common Renderer Conventions

The following parameters are commonly recognized by SevenMark renderers:

| Parameter | Description | Common Usage |
|-----------|-------------|--------------|
| `#style` | Raw inline CSS declarations | Styled blocks, tables, lists, media wrappers |
| `#color` | Text color | Styled blocks and style-aware renderers |
| `#bgcolor` | Background color | Styled blocks and style-aware renderers |
| `#size` | Font size | Styled blocks and style-aware renderers |
| `#opacity` | Opacity | Styled blocks and style-aware renderers |
| `#class` | Extra CSS class names | Many block and media renderers |
| `#dark-style` | Raw dark-mode CSS declarations | Visual renderers that emit `data-dk` and participate in the shared dark-style registry |
| `#dark-color` | Dark-mode text color | Visual renderers that emit `data-dk` and participate in the shared dark-style registry |
| `#dark-bgcolor` | Dark-mode background color | Visual renderers that emit `data-dk` and participate in the shared dark-style registry |
| `#dark-size` | Dark-mode font size | Visual renderers that emit `data-dk` and participate in the shared dark-style registry |
| `#dark-opacity` | Dark-mode opacity | Visual renderers that emit `data-dk` and participate in the shared dark-style registry |

`#dark-*` parameters are separate from provider-specific flags like `#dark` on some external media embeds such as Spotify or Discord.

## Element-specific Parameters

### Tables

Table-level parameters:

- `#caption`: render a `<caption>`
- `#align`: align the table wrapper (`left`, `center`, or `right`)
- `#sortable`: emit `data-sortable="true"` for sortable-table behavior

Row-level parameters:

- `#head`: render the row inside `<thead>` with `<th>` cells

Cell-level parameters:

- `#x`: colspan
- `#y`: rowspan

```sevenmark
{{{#table #caption="Inventory" #align="right" #sortable
[[#head [[Product]] [[Price]]]]
[[[[#x="2" Featured item]] [[In stock]]]]
}}}
```

### Media

Media elements use parameters such as:

- `#file`, `#url`, `#document`, `#category`, `#user`: target selection
- `#anchor`: append a fragment to the resolved link target
- `#theme`: annotate output with `data-theme="light|dark"`

```sevenmark
[[#document="Guide" #anchor="installation" Jump to installation]]
[[#file="logo-dark.svg" #theme="dark" Dark logo]]
```

### Footnotes

Footnotes commonly use:

- `#display`: custom marker text for unnamed footnotes
- `#name`: reusable named footnote identifier

```sevenmark
Text{{{#fn #display="*" Note with custom marker. }}}.
Again{{{#fn #name="api-limit" Shared note. }}}.
```

### Code and CSS Blocks

Common parameters include:

- `#lang` on `{{{#code}}}` for syntax highlighting
- `#class` on `{{{#code}}}` and `{{{#css}}}`
- `#dark-*` on visual renderers such as `{{{#code}}}` to populate shared dark-mode styling rules

`{{{#css}}}` does not consume `#dark-*`; write dark-mode selectors directly in the authored stylesheet.

```sevenmark
{{{#code #lang="rust" #class="example" #dark-bgcolor="#111" #dark-color="#eee"
fn main() {}
}}}
```

## Parameter Combinations

Multiple parameters can be combined:

```sevenmark
{{{ #style="border:2px solid blue; padding:8px" #color="red" #size="18px"
Multi-styled text
}}}

[[#url="https://example.com/docs" #anchor="api" #theme="light" Docs]]
```

## Nested Parameter Usage

Parameters work in nested structures:

```sevenmark
{{{#table #caption="Nested examples"
[[#head [[Name]] [[Content]]]]
[[[[Code]] [[{{{#code #lang="python" #class="nested"
print("hello")
}}}
]]]]
[[[[Media]] [[[[#file="image.png" #theme="dark" Example]]]]]]
}}}
```

</div>
