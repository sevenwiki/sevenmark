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

`#css` accepts common block parameters such as `#class` and `#dark`:

```sevenmark
{{{#css #class="global-theme" #dark="background:#111;color:#eee"
.label { color: #333; }
}}}
```

## Raw Close Rules

`#css` uses the same raw close rule as `#code` and `#tex`:

1. Raw parsing uses triple-brace depth matching (`{{{` increments depth, `}}}` decrements depth).
2. The block closes when depth returns to zero.
3. To avoid ambiguous endings when content ends with `}`, formatter inserts a separator before the final `}}}`.

Example:

```sevenmark
{{{#css
.profile-card { border: 1px solid #ddd; } /* normal CSS */
}}}
```

## Notes

- CSS content is parsed as raw text (no nested SevenMark parsing inside).
- Renderer sanitizes style-close tags (e.g. `</style>`) to prevent tag break-out.

</div>
