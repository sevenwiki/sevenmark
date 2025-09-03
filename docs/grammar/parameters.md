# Parameters

<div v-pre>

SevenMark uses a parameter system to customize element appearance and behavior.

## Parameter Syntax

Parameters are specified using `#parameter="value"` or `#parameter` for boolean flags:

```sevenmark
{{{ #style="color:red" #size="16px" Red text }}}
{{{#list #1 #style="margin-left:20px" 
[[Item 1]]
[[Item 2]]
}}}
```

## Common Style Parameters

### Color

Set text color:

```sevenmark
{{{ #color="blue" Blue text }}}
{{{ #color="red" Red text }}}
```

### Background Color

Set background color:

```sevenmark
{{{ #bg_color="yellow" Text with yellow background }}}
```

### Style

Apply custom CSS styles:

```sevenmark
{{{ #style="font-weight:bold; text-decoration:underline" Styled text }}}
```

### Size

Set font size:

```sevenmark
{{{ #size="20px" Large text }}}
{{{ #size="12px" Small text }}}
```

### Opacity

Set opacity level:

```sevenmark
{{{ #opacity="0.5" Semi-transparent text }}}
```

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