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

## Multiline Comments

Use `/* */` for comments that span multiple lines:

```sevenmark
Text before comment /* This is a
multiline comment
that spans multiple lines */ text after comment.
```

## Comments in Complex Structures

Comments can be used within other elements:

```sevenmark
{{{#list #1
[[First item]] // Comment about first item
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
- Comments can appear anywhere in the document
- Nested comments are not supported (you cannot put `/* */` inside another `/* */`)

</div>