# Footnotes

<div v-pre>

Footnotes allow you to add references, citations, or additional information without cluttering the main text.

## Basic Footnote Structure

Footnotes consist of two parts:

1. **Reference marker** `[fn]` - Placed in the text where you want the footnote number
2. **Footnote content** `{{{#fn}}}` - Contains the actual footnote text

```sevenmark
This statement needs a citation[fn].

{{{#fn This is the footnote content providing the citation. }}}
```

## Defining Footnotes

Use `{{{#fn}}}` to define footnote content:

```sevenmark
{{{#fn This is a footnote with additional information. }}}
{{{#fn Source: Wikipedia, accessed January 2024. }}}
{{{#fn See Smith, J. (2023) for more details. }}}
```

## Footnote References

Use `[fn]` to insert a footnote reference marker:

```sevenmark
This statement requires citation[fn] and further explanation[fn].

{{{#fn Citation for the first reference. }}}
{{{#fn Additional explanation for the second reference. }}}
```

The footnote markers are automatically numbered in order of appearance.

## Complete Example

```sevenmark
# Research Paper

SevenMark is a powerful markup language[fn] designed for wiki systems[fn].

## Implementation Details

The parser uses winnow combinators[fn] for efficient parsing.

## References

{{{#fn SevenMark was first released in 2024. }}}
{{{#fn It powers the SevenWiki platform and related tools. }}}
{{{#fn winnow is a Rust parser combinator library. See: https://docs.rs/winnow }}}
```

## Footnotes with Formatting

Footnote content can include other SevenMark elements:

```sevenmark
Main text with a reference[fn].

{{{#fn
See **Smith, J.** (2023). *Advanced Markup Languages*.
Available at [[#url="https://example.com" example.com]].
}}}
```

### Rich Footnote Content

```sevenmark
The algorithm has O(n log n) complexity[fn].

{{{#fn
Time complexity analysis:
- Best case: {{{#tex O(n) }}}
- Average case: {{{#tex O(n \log n) }}}
- Worst case: {{{#tex O(n^2) }}}

See algorithm textbook for proof.
}}}
```

## Multiple Footnotes

```sevenmark
# Documentation

SevenMark[fn] supports various elements[fn] including footnotes[fn].

{{{#fn A Domain Specific Language for wiki systems. }}}
{{{#fn Text styles, lists, tables, code blocks, and more. }}}
{{{#fn This very feature you're reading about! }}}
```

## Footnotes in Complex Structures

### In Lists

```sevenmark
{{{#list #1
[[First item with citation[fn]]]
[[Second item with reference[fn]]]
[[Third item without footnote]]
}}}

{{{#fn Citation for first item. }}}
{{{#fn Reference for second item. }}}
```

### In Tables

```sevenmark
{{{#table
[[[[Feature]] [[Status]]]]
[[[[Parser[fn]]] [[Complete]]]]
[[[[Serialization[fn]]] [[Complete]]]]
}}}

{{{#fn Uses winnow parser combinators. }}}
{{{#fn Supports JSON output via serde. }}}
```

### In Headers

```sevenmark
## The SevenMark Language[fn]

{{{#fn Created by the SevenWiki team in 2024. }}}
```

## Academic/Technical Writing

### Citations

```sevenmark
Recent studies[fn] have shown that parser combinators[fn] provide
excellent performance characteristics.

{{{#fn Johnson et al. (2023). "Performance Analysis of Parser Combinators".
Journal of Programming Languages, 15(3), 234-256. }}}

{{{#fn See Wadler (1995) for the original monadic parser combinator paper. }}}
```

### Multiple References

```sevenmark
The technique is well-documented[fn][fn][fn] in the literature.

{{{#fn Smith (2020), Chapter 5. }}}
{{{#fn Jones (2021), pp. 45-67. }}}
{{{#fn Brown (2023), Section 3.2. }}}
```

## Footnote Numbering

Footnotes are automatically numbered:

```sevenmark
First footnote[fn], second footnote[fn], third footnote[fn].

{{{#fn Footnote #1 }}}
{{{#fn Footnote #2 }}}
{{{#fn Footnote #3 }}}
```

Numbers (1, 2, 3...) are assigned in the order that `[fn]` markers appear.

## Long Footnotes

Footnotes can contain multiple paragraphs:

```sevenmark
See the detailed explanation[fn] for more information.

{{{#fn
This is a long footnote with multiple paragraphs.

The first paragraph provides the main point.

The second paragraph adds additional context and examples.

The third paragraph summarizes and provides references.
}}}
```

## Footnote Placement

### Inline (Adjacent)

```sevenmark
Important statement[fn].
{{{#fn Immediate footnote definition. }}}

Next statement[fn].
{{{#fn Another immediate definition. }}}
```

### End of Section

```sevenmark
# Section Title

Content with references[fn] and citations[fn].

More content here[fn].

## References

{{{#fn First reference. }}}
{{{#fn Second reference. }}}
{{{#fn Third reference. }}}
```

### End of Document

```sevenmark
# Article

Content throughout the document[fn] with various references[fn].

## More Content

Additional sections with more footnotes[fn].

---

## Footnotes

{{{#fn First footnote at document end. }}}
{{{#fn Second footnote at document end. }}}
{{{#fn Third footnote at document end. }}}
```

## Technical Restrictions

### No Nested Footnotes

Footnotes cannot be nested inside other footnotes:

```sevenmark
<!-- Valid -->
Main text[fn].
{{{#fn Footnote content. }}}

<!-- Invalid -->
{{{#fn Footnote content[fn] with another footnote. }}}
```

### Parser Behavior

The parser prevents footnote nesting:

- `[fn]` markers inside `{{{#fn}}}` content are treated as literal text
- This prevents infinite recursion and keeps footnotes simple

## Best Practices

### When to Use Footnotes

 **Good uses:**
- Citations and references
- Additional explanations that would break text flow
- Source attributions
- Technical details for interested readers
- Asides and tangential information

L **Avoid:**
- Essential information (put it in main text)
- Very long footnotes (consider an appendix)
- Too many footnotes (clutters the text)

### Writing Style

```sevenmark
<!-- Good: Concise footnotes -->
SevenMark uses parser combinators[fn].
{{{#fn Built with winnow 0.7.13. }}}

<!-- Less ideal: Too verbose -->
SevenMark[fn] is great.
{{{#fn SevenMark is a domain-specific language created by the SevenWiki
team in 2024, designed for wiki systems, implementing a sophisticated parser
using winnow combinators with comprehensive error handling... }}}
```

## Styling

Footnote content supports parameters:

```sevenmark
See reference[fn].

{{{#fn #style="font-size: 0.9em; color: #666"
Styled footnote content.
}}}
```

## Technical Notes

- Footnote references use `[fn]` syntax (square brackets)
- Footnote definitions use `{{{#fn}}}` syntax (triple braces)
- The reference marker (`[fn]`) can appear anywhere in the text
- Footnote content can contain most SevenMark elements
- Footnotes cannot be nested (no `[fn]` inside `{{{#fn}}}`)
- Numbering is automatic and based on order of appearance
- During preprocessing, footnotes are collected for processing by the wiki system

</div>