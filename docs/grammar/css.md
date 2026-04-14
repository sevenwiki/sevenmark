# CSS Blocks

<div v-pre>

SevenMark supports raw CSS blocks with `{{{#css ... }}}`.

## Basic Usage

```sevenmark
{{{#css
.profile-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 12px;
}
}}}
```

## Parameters

`#css` does not accept parameters. Write raw stylesheet content directly after `#css`.

For dark-mode CSS, author it directly in the stylesheet with selectors such as `.dark .label { ... }` or with `@media (prefers-color-scheme: dark) { ... }`.

## Raw Close Rules

`#css` uses the same raw close rule as `#code` and `#tex`:

1. Raw parsing uses triple-brace depth matching (`{{{` increments depth, `}}}` decrements depth).
2. The block closes when depth returns to zero.
3. To avoid ambiguous endings when content ends with `}`, the formatter may insert a separator before the final `}}}`.

Example:

```sevenmark
{{{#css
.profile-card { border: 1px solid #ddd; }
}}}
```

## Notes

- CSS content is parsed as raw text; nested SevenMark syntax is not processed inside the block.
- Renderer sanitizes style-close tags such as `</style>` to prevent tag break-out.

</div>
