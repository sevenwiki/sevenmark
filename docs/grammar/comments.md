# Comments

<div v-pre>

SevenMark supports both inline and multiline comments that are not rendered in the output.

## Inline Comments

Use `//` for single-line comments:

```sevenmark
Text before comment // This is an inline comment
Text after comment.
```

The comment extends from `//` to the end of the line and is not displayed in the rendered output.

::: warning
Inline comments (`//`) only work at the **top level** of a document. They are disabled inside nested constructs such as `{{{ }}}`, `[[ ]]`, etc. This prevents `//` in URLs and other content from being incorrectly consumed as comments.

```sevenmark
// This works (top-level)
{{{#list #1
[[Item with https://example.com link]]  // This is NOT a comment — it becomes plain text
}}}
```

Use `/* */` multiline comments instead if you need comments inside nested structures.
:::

## Multiline Comments

Use `/* */` for comments that span multiple lines:

```sevenmark
Text before comment /* This is a
multiline comment
that spans multiple lines */ text after comment.
```

Multiline comments work at any nesting level, including inside tables, lists, folds, and other constructs.

## Comments in Complex Structures

Multiline comments can be used within other elements:

```sevenmark
{{{#list #1
[[First item]]
[[Second item /* inline multiline comment */ with continuation]]
[[Third item]]
}}}

/* This is a comment explaining
   the structure of this wiki page:
   - Category classification
   - Headers with different levels
   - Mixed formatting styles
   - Nested complex elements
*/
```

## Comment Usage Guidelines

- Use comments to explain complex markup structures
- Comments are useful for temporary notes during document editing
- Comments are completely removed from the rendered output
- Inline comments (`//`) are only available at the top level (not inside `{{{ }}}`, `[[ ]]`, etc.)
- Multiline comments (`/* */`) can appear anywhere in the document, including inside nested structures
- Nested comments are not supported (you cannot put `/* */` inside another `/* */`)

</div>