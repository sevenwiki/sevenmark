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

1. The block closes at the first `}}}` sequence.
2. A literal `}}}` cannot appear inside CSS content.
3. Prefer splitting text to avoid producing `}}}` inside raw CSS.

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
