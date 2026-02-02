# Media Elements

<div v-pre>

SevenMark uses `[[]]` syntax for media elements like images, links, and files.

## Basic Media Syntax

### Images

Embed images using the `#file` parameter:

```sevenmark
[[#file="image.png" Alt text for image]]
[[#file="screenshot.jpg" Application Screenshot]]
```

### External URLs

Link to external media using the `#url` parameter:

```sevenmark
[[#url="https://example.com/image.jpg" External image]]
[[#url="https://example.com/video.mp4" Demo Video]]
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
[[#document="API Reference" API Documentation]]
[[#document="Tutorial" Getting Started Guide]]
```

### Category Links

Link to category pages using the `#category` parameter:

```sevenmark
[[#category="Programming Languages" Category Page]]
[[#category="Rust" All Rust Articles]]
```

## Hyperlinks

Create hyperlinks using the `#url` parameter:

```sevenmark
[[#url="https://example.com"]]
[[#url="https://rust-lang.org" Official Rust Website]]
[[#url="https://github.com/rust-lang/rust" Rust GitHub Repository]]
```

The first element is the URL (using `#url=`), and the second element is optional display text. If no display text is provided, the URL itself will be displayed.

## Media in Complex Structures

### Media in Tables

```sevenmark
{{{#table
[[[[Name]] [[Image]] [[Link]]]]
[[[[John]] [[[#file="john.jpg" John's photo]]] [[[#url="john.com" Profile]]]]]
}}}
```

### Media in Lists

```sevenmark
{{{#list #1
[[Profile images:]]
[[[[#file="avatar1.png" User 1]]]]
[[[[#file="avatar2.png" User 2]]]]
[[Documentation: [[#url="https://docs.example.com" Official Docs]]]]
}}}
```

## File Types

SevenMark media elements can handle various file types:

```sevenmark
[[#file="document.pdf" PDF Document]]
[[#file="presentation.pptx" PowerPoint Presentation]]  
[[#file="data.xlsx" Excel Spreadsheet]]
[[#file="video.mp4" Video File]]
[[#file="audio.mp3" Audio File]]
```

## Media Parameter Priority

When multiple parameters are specified, they are resolved in this priority order:

1. `#file` - Highest priority
2. `#document` - Second priority
3. `#category` - Third priority
4. `#url` - Lowest priority (fallback)

Example:

```sevenmark
// This will use #file (highest priority)
[[#file="image.png" #url="https://example.com/image.jpg" My Image]]

// This will use #document (only #document and #url specified)
[[#document="HomePage" #url="https://example.com" Home Link]]
```

The preprocessing stage resolves media references to actual URLs:
- **File namespace**: Resolved to storage URLs via API
- **Document namespace**: Generates `/document/{title}` links
- **Category namespace**: Generates `/category/{title}` links
- **URL parameter**: Passes through unchanged

## External Media Embeds

For embedding external media from platforms like YouTube, Vimeo, Spotify, and more, see [External Media Embeds](./external-media.md).

Quick examples:

```sevenmark
// YouTube video
[[#youtube #id="dQw4w9WgXcQ"]]

// Spotify track
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC"]]

// Discord server widget
[[#discord #id="123456789012345678"]]
```

## Advanced Usage

### Media with Styling

While media elements themselves don't support direct styling, they can be wrapped in styled elements:

```sevenmark
{{{ #style="text-align: center; border: 1px solid #ccc; padding: 10px"
[[#file="important-diagram.png" System Architecture Diagram]]
}}}
```

</div>