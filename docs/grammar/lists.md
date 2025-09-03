# Lists

<div v-pre>

SevenMark uses `{{{#list}}}` syntax for creating lists with various numbering styles.

## Basic Lists

### Numeric List (1, 2, 3...)

```sevenmark
{{{#list #1
[[Item 1]]
[[Item 2]]
[[Item 3]]
}}}
```

### Lowercase Alphabetic List (a, b, c...)

```sevenmark
{{{#list #a
[[First item]]
[[Second item]]
[[Third item]]
}}}
```

### Uppercase Alphabetic List (A, B, C...)

```sevenmark
{{{#list #A
[[First item]]
[[Second item]]
[[Third item]]
}}}
```

### Lowercase Roman Numerals (i, ii, iii...)

```sevenmark
{{{#list #i
[[First item]]
[[Second item]]
[[Third item]]
}}}
```

### Uppercase Roman Numerals (I, II, III...)

```sevenmark
{{{#list #I
[[First item]]
[[Second item]]
[[Third item]]
}}}
```

## Nested Lists

Lists can be nested within other lists:

```sevenmark
{{{#list #1 #style="margin-left:20px"
[[**Chapter 1**: Introduction]]
[[Subsection: {{{#list #a [[Point A with *emphasis*]] [[Point B with [now]]] }}} ]]
[[**Chapter 2**: Advanced Topics]]
}}}
```

## Styled Lists

### List-wide Styling

```sevenmark
{{{#list #1 #style="color:blue; margin-left:20px"
[[Blue colored item 1]]
[[Blue colored item 2]]
}}}
```

### Complex List Example

```sevenmark
{{{#list #1 #style="margin-left:20px"
[[**Chapter 1**: Introduction]]
[[Subsection: {{{#list #a [[Point A with *emphasis*]] [[Point B with [now]]] }}} ]]
[[**Chapter 2**: Advanced Topics]]
[[Complex item: {{{#fold [[Click here]] [[Hidden: {{{#code #lang="rust" fn main() { println!("Deep!"); } }}}]] }}} ]]
[[Final chapter with ^^superscript^^ text]]
}}}
```

## Lists with Rich Content

List items can contain any SevenMark elements:

```sevenmark
{{{#list #1
[[Text styles: **bold**, *italic*, __underline__]]
[[Code: {{{#code println!("Hello!"); }}}]]
[[Tables: {{{#table [[[[Item]] [[Value]]]] [[[[A]] [[1]]]] [[[[B]] [[2]]]] }}}]]
[[Media: [[#file="example.png" Example image]]]]
}}}
```

</div>