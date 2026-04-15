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

The structure is:

- `{{{#table}}}`: table container
- `[[ ... ]]`: one row
- `[[Cell]]`: one cell inside that row

## Header Rows (`#head`)

Add the `#head` flag on a row to render it inside `<thead>` with `<th>` cells.

```sevenmark
{{{#table
[[#head [[Product]] [[Price]] [[Stock]]]]
[[[[Laptop]] [[$1,200]] [[5 units]]]]
[[[[Mouse]] [[$30]] [[20 units]]]]
}}}
```

Rows without `#head` render inside `<tbody>` with normal `<td>` cells.

## Table Caption, Sorting, and Alignment

Use `#caption` on the table element to render a `<caption>`, `#sortable` to opt into sortable-table behavior on the frontend, and `#wrapper-align` to position the table wrapper.

```sevenmark
{{{#table #caption="Inventory" #wrapper-align="right" #sortable
[[#head [[Product]] [[Price]] [[Stock]]]]
[[[[Laptop]] [[$1,200]] [[5 units]]]]
[[[[Mouse]] [[$30]] [[20 units]]]]
}}}
```

`#wrapper-align` accepts `left`, `center`, and `right`. It affects the outer table wrapper rather than the `<table>` element itself, so authored table styles can stay on `#style` while layout is handled separately.

Use `#wrapper-width` to set a fixed width on the wrapper div, and `#wrapper-style` / `#wrapper-dark-style` for arbitrary CSS on the wrapper:

```sevenmark
{{{#table #wrapper-align="right" #wrapper-width="400px" #wrapper-style="margin:2rem 0"
[[#head [[Product]] [[Price]]]]
[[[[Laptop]] [[$1,200]]]]
}}}
```

## Cell Merging

### Horizontal Merge (`#x`)

```sevenmark
{{{#table
[[[[#x="2" Merged cell spanning 2 columns]] [[Normal cell]]]]
[[[[Cell 1]] [[Cell 2]] [[Cell 3]]]]
}}}
```

### Vertical Merge (`#y`)

```sevenmark
{{{#table
[[[[#y="2" Merged cell spanning 2 rows]] [[Cell 1,2]]]]
[[[[ ]] [[Cell 2,2]]]]
}}}
```

## Styling

Tables, rows, and cells support the common style parameters such as `#style`, `#class`, `#color`, `#bgcolor`, and the `#dark-*` overrides.

### Table-level Styling

```sevenmark
{{{#table #style="width:100%; border-collapse:collapse" #dark-bgcolor="#111"
[[#head [[Name]] [[Status]]]]
[[[[Widget A]] [[Active]]]]
}}}
```

### Cell Styling

```sevenmark
{{{#table
[[#head [[Name]] [[Status]]]]
[[[[#color="blue" Laptop]] [[#bgcolor="lightyellow" Ready]]]]
}}}
```

## Conditional Rows and Cells

Tables support conditional rendering at both row and cell level using `{{{#if}}}`.

### Conditional Rows

```sevenmark
{{{#define #showDetails="true"}}}

{{{#table
[[#head [[Product]] [[Price]]]]
[[[[Widget A]] [[$10]]]]
{{{#if [var(showDetails)] == "true" :: [[[[Details]] [[Size: Medium]]]] }}}
[[[[Widget B]] [[$20]]]]
}}}
```

### Conditional Cells

```sevenmark
{{{#define #showStock="true"}}}

{{{#table
[[#head [[Product]] [[Price]] {{{#if [var(showStock)] == "true" :: [[Stock]] }}}]]
[[[[Widget A]] [[$10]] {{{#if [var(showStock)] == "true" :: [[5 units]] }}}]]
[[[[Widget B]] [[$20]] {{{#if [var(showStock)] == "true" :: [[10 units]] }}}]]
}}}
```

## Nested Markup

Table cells can contain other SevenMark elements:

```sevenmark
{{{#table #caption="Feature matrix"
[[#head [[Feature]] [[Description]]]]
[[[[**Bold**]] [[*Italic* text]]]]
[[[[Code]] [[{{{#code
inline_code()
}}}
]]]]
[[[[Anchor]] [[[anchor(api-section)]]]]]
[[[[Media]] [[[[#file="image.png" Image]]]]]]
}}}
```

Note: use `[[#file="..."]]`, `[[#url="..."]]`, and other media syntax inside cells, not `@media`.

</div>
