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
[[[[#bgcolor="yellow" Yellow background]] [[Normal cell]]]]
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

## Conditional Rows and Cells

Tables support conditional rendering at both row and cell level using `{{{#if}}}` syntax.

### Conditional Rows

Include or exclude entire rows based on a condition:

```sevenmark
{{{#define #showDetails="true"}}}

{{{#table
[[[[Product]] [[Price]]]]
[[[[Widget A]] [[$10]]]]
{{{#if [var(showDetails)] == "true" :: [[[[Details]] [[Size: Medium]]]] }}}
[[[[Widget B]] [[$20]]]]
}}}
```

The conditional row `[[[[Details]] [[Size: Medium]]]]` is included only when `showDetails` is `"true"`.

### Conditional Cells

Include or exclude specific cells within a row:

```sevenmark
{{{#define #showStock="true"}}}

{{{#table
[[ [[Product]] [[Price]] {{{#if [var(showStock)] == "true" :: [[Stock]] }}} ]]
[[ [[Widget A]] [[$10]] {{{#if [var(showStock)] == "true" :: [[5 units]] }}} ]]
[[ [[Widget B]] [[$20]] {{{#if [var(showStock)] == "true" :: [[10 units]] }}} ]]
}}}
```

### Multiple Conditional Items

You can include multiple rows or cells in a single conditional:

```sevenmark
{{{#table
[[[[Header 1]] [[Header 2]]]]
{{{#if [var(showBoth)] == "true" ::
[[[[Row A1]] [[Row A2]]]]
[[[[Row B1]] [[Row B2]]]]
}}}
[[[[Footer 1]] [[Footer 2]]]]
}}}
```

### Conditional with Complex Expressions

```sevenmark
{{{#define #userRole="admin"}}}
{{{#define #showSensitive="true"}}}

{{{#table
[[[[Name]] [[Email]] [[Actions]]]]
[[[[John]] [[john@example.com]] [[View]]]]
{{{#if [var(userRole)] == "admin" && [var(showSensitive)] == "true" ::
[[[[Admin Data]] [[admin@internal]] [[Delete]]]]
}}}
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
