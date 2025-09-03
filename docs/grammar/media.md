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

## Hyperlinks

### Simple Links

For simple URLs, you can use the content as the URL:

```sevenmark
[[https://example.com]]
[[Click here to visit the site]]
```

### Links with Display Text

```sevenmark
[[#url="https://rust-lang.org" Official Rust Website]]
[[#url="https://github.com/rust-lang/rust" Rust GitHub Repository]]
```

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

## Advanced Usage

### Media with Styling

While media elements themselves don't support direct styling, they can be wrapped in styled elements:

```sevenmark
{{{ #style="text-align: center; border: 1px solid #ccc; padding: 10px"
[[#file="important-diagram.png" System Architecture Diagram]]
}}}
```

</div>