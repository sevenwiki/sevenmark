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

1. The block closes only when a line is exactly `}}}` (leading/trailing spaces allowed).
2. `}}}` in the middle of a line is treated as CSS text.
3. If you need a literal line-only closer, write `\}}}` (parsed as `}}}` content).

Example:

```sevenmark
{{{#css
.a::after { content: "}}}"; } /* not a closer */
\}}}                          /* literal line-only closer */
}}}                           /* actual close */
```

## Notes

- CSS content is parsed as raw text (no nested SevenMark parsing inside).
- Renderer sanitizes style-close tags (e.g. `</style>`) to prevent tag break-out.

</div>
