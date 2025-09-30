# Tables

<div v-pre>

SevenMark uses `{{{#table}}}` syntax for tables, with rows and cells structured using nested `[[]]` brackets.

## Basic Table

```sevenmark
{{{#table
[[[[Cell 1,1]] [[Cell 1,2]]]]
[[[[Cell 2,1]] [[Cell 2,2]]]]
}}}
```

The structure is as follows:
- `{{{#table}}}`: Table container
- `[[[[Cell]] [[Cell]]]]`: Table row (TableInnerElement1)
- Inner `[[Cell]]`: Individual cell (TableInnerElement2)

## Styled Tables

```sevenmark
{{{#table #style="border-collapse:collapse"
[[[[**Header 1**]] [[*Header 2*]] [[~~Header 3~~]]]]
[[[[Simple cell]] [[Styled cell]] [[Another cell]]]]
}}}
```

## Cell Merging

### Horizontal Merge (colspan)

Use the `#x` parameter:

```sevenmark
{{{#table
[[[[#x="2" Merged cell spanning 2 columns]] [[Normal cell]]]]
[[[[Cell 1]] [[Cell 2]] [[Cell 3]]]]
}}}
```

### Vertical Merge (rowspan)

Use the `#y` parameter:

```sevenmark
{{{#table
[[[[#y="2" Merged cell spanning 2 rows]] [[Cell 1,2]]]]
[[[[ ]] [[Cell 2,2]]]]
}}}
```

## Table Styling

### Table-Level Styling

```sevenmark
{{{#table #style="border: 2px solid #333;"
[[[[Header 1]] [[Header 2]]]]
[[[[Cell 1]] [[Cell 2]]]]
}}}
```

### Individual Cell Styling

```sevenmark
{{{#table
[[[[Header 1]] [[Header 2]]]]
[[[[#color="red" Red text]] [[Normal cell]]]]
[[[[#bg_color="yellow" Yellow background]] [[Normal cell]]]]
}}}
```

## Complex Table Example

```sevenmark
{{{#table #style="width: 100%; border-collapse: collapse;"
[[[[#style="text-align: center; font-weight: bold;" Product]] [[Price]] [[Stock]]]]
[[[[#color="blue" Laptop]] [[#style="text-align: right;" $1,200]] [[5 units]]]]
[[[[#color="green" Mouse]] [[#style="text-align: right;" $30]] [[20 units]]]]
[[[[#x="2" #style="text-align: center; font-weight: bold;" Total]] [[#style="text-align: right; font-weight: bold;" $1,230]]]]
}}}
```

## Nested Markup

Table cells can contain other SevenMark elements:

```sevenmark
{{{#table
[[[[Feature]] [[Description]]]]
[[[[**Bold**]] [[*Italic* text]]]]
[[[[{{{#code inline_code() }}}]] [[Code is supported]]]]
[[[[[[#file="image.png" Image]]]] [[Media elements work too]]]]
}}}
```

Note: Use `[[#file="..."]]` or `[[#url="..."]]` for media elements in tables, not `@media`.

</div>