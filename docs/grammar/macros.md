# Macros

<div v-pre>

SevenMark supports various macro elements for dynamic content generation.

## Time Macros

### Current Time

Display the current timestamp:

```sevenmark
Current time: [now]
```

### Age Calculation

Calculate age from a given date:

```sevenmark
Age calculation: [age(1990-01-15)]
Born in 1995: [age(1995-06-20)]
```

## Utility Macros

### Line Break

Insert a line break:

```sevenmark
Line break:[br]Next line
```

### Null Macro

The null macro produces no output:

```sevenmark
This text[null]continues without interruption.
```

## Macro Usage in Complex Elements

Macros can be used within other elements:

```sevenmark
{{{#list #1
[[Item with current time: [now]]]
[[Person born in 1990 is [age(1990-01-01)] years old]]
[[Line 1[br]Line 2 in same item]]
}}}
```

### In Tables

```sevenmark
{{{#table
[[[[Name]] [[Age]]]]
[[[[John]] [[[age(1985-03-15)]]]]]
[[[[Updated]] [[[now]]]]]
}}}
```

### In Styled Text

```sevenmark
{{{ #style="color:blue" Last updated: [now] }}}
```

</div>