# Media Elements

<div v-pre>

SevenMark uses `[[]]` syntax for media elements like images, links, and files.

## Basic Media Syntax

### Images

Embed images using the `#file` parameter:

```sevenmark
[[#file="image.png" Alt text for image]]
[[#file="screenshot.jpg" Application screenshot]]
```

### External URLs

Link to external media using the `#url` parameter. `#url` only renders when the value starts with `http://` or `https://`.

```sevenmark
[[#url="https://example.com/image.jpg" External image]]
[[#url="https://example.com/video.mp4" Demo video]]
```

### Combined File and URL

Provide both local file and backup URL:

```sevenmark
[[#file="document.pdf" #url="https://backup.com/doc.pdf" PDF document]]
```

### Wiki Page Links

Link to wiki pages using the `#document` parameter:

```sevenmark
[[#document="HomePage" Home]]
[[#document="API Reference" API documentation]]
[[#document="Tutorial" Getting started guide]]
```

### Category and User Links

```sevenmark
[[#category="Programming Languages" Category page]]
[[#user="Alice" Alice's page]]
```

## Anchor Fragments (`#anchor`)

Add `#anchor` to append a fragment identifier to the resolved link target.

```sevenmark
[anchor(installation)]
## Installation

[[#document="Guide" #anchor="installation" Jump to installation]]
[[#url="https://example.com/docs" #anchor="faq" Open external FAQ]]
```

This is especially useful when combined with `[anchor(name)]`.

## Theme-aware Media (`#theme`)

Use `#theme="light"` or `#theme="dark"` to annotate a media element with a theme hint for frontend CSS.

```sevenmark
[[#file="logo-light.svg" #theme="light" Light logo]]
[[#file="logo-dark.svg" #theme="dark" Dark logo]]
```

The renderer exposes the value as a `data-theme` attribute. Values other than `light` and `dark` are ignored.

## Hyperlinks

Create hyperlinks using the `#url` parameter:

```sevenmark
[[#url="https://example.com"]]
[[#url="https://rust-lang.org" Official Rust website]]
[[#url="https://github.com/rust-lang/rust" Rust GitHub repository]]
```

If no display text is provided, the URL itself is displayed.

## Media in Complex Structures

### Media in Tables

```sevenmark
{{{#table
[[#head [[Name]] [[Image]] [[Link]]]]
[[[[John]] [[[#file="john.jpg" John's photo]]] [[[#url="https://john.com" Profile]]]]]
}}}
```

### Media in Lists

```sevenmark
{{{#list #1
[[Profile images:]]
[[[[#file="avatar1.png" User 1]]]]
[[[[#file="avatar2.png" User 2]]]]
[[Documentation: [[#url="https://docs.example.com" Official docs]]]]
}}}
```

## File Types

SevenMark media elements can handle many file types:

```sevenmark
[[#file="document.pdf" PDF document]]
[[#file="presentation.pptx" PowerPoint presentation]]
[[#file="data.xlsx" Excel spreadsheet]]
[[#file="video.mp4" Video file]]
[[#file="audio.mp3" Audio file]]
```

## Resolution and Link Priority

When multiple target parameters are specified:

1. `#file` controls image or file rendering.
2. Link target priority is `#url` > `#document` > `#category` > `#user`.
3. If `#anchor` is present, the fragment is appended after the final target URL is chosen.

Example:

```sevenmark
// Renders image from #file, links to #url, then appends #details
[[#file="image.png" #url="https://example.com/image" #anchor="details" My image]]

// Uses #document when #url is not provided
[[#document="HomePage" Home link]]
```

The preprocessing stage resolves media references to actual URLs:

- `#file`: resolved to storage URLs
- `#document`: rendered as document links
- `#category`: rendered as category links
- `#user`: rendered as user links
- `#url`: allowed only for `http://` and `https://`

## External Media Embeds

For embedding external media from platforms like YouTube, Vimeo, Spotify, and more, see [External Media Embeds](./external-media.md).

Quick examples:

```sevenmark
[[#youtube #id="dQw4w9WgXcQ"]]
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC"]]
[[#discord #id="123456789012345678"]]
```

## Advanced Usage

### Media with Styling

Media wrappers support common styling parameters such as `#style`, `#class`, and `#dark-*`.

```sevenmark
{{{ #style="text-align:center; border:1px solid #ccc; padding:10px"
[[#file="important-diagram.png" System architecture diagram]]
}}}
```

</div>
