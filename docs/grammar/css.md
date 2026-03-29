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

`#css` accepts `#class` plus structured dark-mode overrides such as `#dark-style`, `#dark-color`, `#dark-bgcolor`, `#dark-size`, and `#dark-opacity`.

```sevenmark
{{{#css #class="global-theme" #dark-style="background:#111" #dark-color="#eee"
.label { color: #333; }
}}}
```

These dark parameters are attached to the rendered `<style>` element as `data-dark-style`; they do not rewrite the CSS source text inside the block.

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
