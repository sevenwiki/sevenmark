# Ruby Text (Japanese Annotations)

<div v-pre>

Ruby text provides pronunciation guides or annotations above base text, commonly used in Japanese (furigana) and East Asian typography.

## Basic Ruby

Use `{{{#ruby}}}` syntax to create ruby annotations:

```sevenmark
{{{#ruby "W }}}
{{{#ruby q� }}}
{{{#ruby �,� }}}
```

## Ruby in Context

Ruby text is typically used inline within sentences:

```sevenmark
The capital of Japan is {{{#ruby q� }}} (Tokyo).

Reading {{{#ruby "W }}} can be challenging for learners.

I'm studying {{{#ruby �,� }}} at university.
```

## Styled Ruby

Apply styling using parameters:

```sevenmark
{{{#ruby #style="color: red" �, }}}
{{{#ruby #style="font-size: 1.2em" "W }}}
{{{#ruby #color="blue" q� }}}
```

## Common Japanese Examples

### Kanji with Readings

```sevenmark
Common words:
- {{{#ruby f! }}} (gakkou - school)
- {{{#ruby H }}} (sensei - teacher)
- {{{#ruby �7 }}} (benkyou - study)
- {{{#ruby 'f }}} (daigaku - university)
```

### Place Names

```sevenmark
Cities in Japan:
- {{{#ruby q� }}} (TMkyM)
- {{{#ruby '* }}} (Lsaka)
- {{{#ruby �� }}} (KyMto)
- {{{#ruby wS }}} (HokkaidM)
```

### Common Phrases

```sevenmark
Basic phrases:
- {{{#ruby S�kao }}} (Hello)
- {{{#ruby B�LhF }}} (Thank you)
- {{{#ruby U�Fj� }}} (Goodbye)
```

## Ruby in Complex Structures

### Ruby in Lists

```sevenmark
{{{#list #1
[[Learn {{{#ruby r�Lj }}} (Hiragana)]]
[[Learn {{{#ruby ���� }}} (Katakana)]]
[[Learn {{{#ruby "W }}} (Kanji)]]
}}}
```

### Ruby in Tables

```sevenmark
{{{#table
[[[[Kanji]] [[Reading]] [[Meaning]]]]
[[[[{{{#ruby 4 }}}]] [[mizu]] [[water]]]]
[[[[{{{#ruby k }}}]] [[hi]] [[fire]]]]
[[[[{{{#ruby ( }}}]] [[ki]] [[tree]]]]
}}}
```

### Ruby in Headers

```sevenmark
# Introduction to {{{#ruby �,� }}}

## Learning {{{#ruby "W }}}

Learn the basics of kanji characters.
```

## Use Cases

### Educational Materials

```sevenmark
Beginner's lesson:

Today we will learn about {{{#ruby �i }}} (animals).

Common {{{#ruby �i }}}:
- {{{#ruby � }}} (dog)
- {{{#ruby + }}} (cat)
- {{{#ruby � }}} (bird)
```

### Literary Text

```sevenmark
{{{#quote
{{{#ruby % }}}n{{{#ruby � }}}oU�U�{{{#ruby A }}}�

Spring streams flow gently.
}}}
```

### Language Learning

```sevenmark
{{{#fold
[[Practice Vocabulary]]
[[
Read these words:
1. {{{#ruby , }}} (hon - book)
2. {{{#ruby B� }}} (jikan - time)
3. {{{#ruby �T }}} (tomodachi - friend)
]]
}}}
```

## Styling Options

All standard parameters are supported:

- `#style` - Custom CSS styling
- `#color` - Text color
- `#size` - Font size
- `#opacity` - Opacity level

```sevenmark
{{{#ruby #style="font-weight: bold; color: darkblue" ́ }}}
```

## Best Practices

### When to Use Ruby

 **Good uses:**
- Educational materials for language learners
- Difficult or uncommon kanji
- Names with non-standard readings
- Literary works targeting beginners

L **Avoid:**
- Common words that readers likely know
- Overusing in advanced materials
- When the base text is already clear

### Placement

```sevenmark
<!-- Good: Natural inline usage -->
Learn to read {{{#ruby �^ }}} every day.

<!-- Less ideal: Excessive ruby -->
{{{#ruby I }}} {{{#ruby want }}} {{{#ruby to }}} {{{#ruby learn }}}...
```

## Cultural Context

Ruby annotations are essential in Japanese typography:

- **Furigana**: Reading guides for kanji
- **Educational**: Helps children and learners
- **Disambiguation**: Clarifies uncommon readings
- **Accessibility**: Aids readers with reading difficulties

Example from children's literature:

```sevenmark
{{{#ruby �KW�KW }}}B�{{{#ruby @ }}}k{{{#ruby * }}}WD{{{#ruby �� }}}L{{{#ruby O }}}�gD~W_

Once upon a time, in a certain place, there lived a kind king.
```

## Technical Notes

- Ruby text uses the `{{{#ruby}}}` syntax with the annotated text as content
- Parameters can be used to style both the base text and annotations
- Ruby elements can be nested within other SevenMark elements
- Empty ruby elements are valid: `{{{#ruby }}}`

</div>