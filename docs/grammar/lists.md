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

## Conditional List Items

Lists support conditional rendering at item level using `{{{#if}}}` syntax.

### Basic Conditional Items

Include or exclude list items based on a condition:

```sevenmark
{{{#define #showAdvanced="true"}}}

{{{#list #1
[[Basic feature]]
[[Standard feature]]
{{{#if [var(showAdvanced)] == "true" :: [[Advanced feature]] }}}
[[Final feature]]
}}}
```

The conditional item `[[Advanced feature]]` is included only when `showAdvanced` is `"true"`.

### Multiple Conditional Items

You can include multiple items in a single conditional:

```sevenmark
{{{#define #isPremium="true"}}}

{{{#list #1
[[Free feature 1]]
[[Free feature 2]]
{{{#if [var(isPremium)] == "true" ::
[[Premium feature A]]
[[Premium feature B]]
[[Premium feature C]]
}}}
[[Common feature]]
}}}
```

### Conditional with Complex Expressions

```sevenmark
{{{#define #userLevel="3"}}}

{{{#list #1
[[Level 1 content]]
{{{#if int([var(userLevel)]) >= 2 :: [[Level 2 content]] }}}
{{{#if int([var(userLevel)]) >= 3 :: [[Level 3 content]] }}}
{{{#if int([var(userLevel)]) >= 4 :: [[Level 4 content]] }}}
}}}
```

### Practical Example: Feature List

```sevenmark
{{{#define #plan="pro"}}}

{{{#list #1
[[✓ Basic support]]
[[✓ 10GB storage]]
{{{#if [var(plan)] == "pro" || [var(plan)] == "enterprise" ::
[[✓ Priority support]]
[[✓ 100GB storage]]
}}}
{{{#if [var(plan)] == "enterprise" ::
[[✓ Dedicated account manager]]
[[✓ Unlimited storage]]
[[✓ Custom integrations]]
}}}
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