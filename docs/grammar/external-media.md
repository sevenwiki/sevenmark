# External Media Embeds

<div v-pre>

SevenMark supports embedding external media from popular platforms using the `[[#provider ...]]` syntax.

## Supported Platforms

| Platform | Syntax | Description |
|----------|--------|-------------|
| YouTube | `[[#youtube ...]]` | Video and playlist embeds |
| Vimeo | `[[#vimeo ...]]` | Video embeds |
| NicoNico | `[[#nicovideo ...]]` | Japanese video platform |
| Spotify | `[[#spotify ...]]` | Music and podcast embeds |
| Discord | `[[#discord ...]]` | Server widget embeds |

---

## YouTube

Embed YouTube videos and playlists.

### Basic Video

```sevenmark
[[#youtube #id="dQw4w9WgXcQ"]]
```

### Playlist

```sevenmark
[[#youtube #playlist="PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"]]
```

### Video from Playlist

```sevenmark
[[#youtube #id="dQw4w9WgXcQ" #playlist="PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"]]
```

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `#id` | Video ID (required if no playlist) | - |
| `#playlist` | Playlist ID | - |
| `#width` | Player width (CSS value, e.g. `800px`, `50%`) | `min(640px,100%)` |
| `#height` | Player height (CSS value) | auto via `aspect-ratio:16/9` |
| `#start` | Start time in seconds | - |
| `#end` | End time in seconds | - |
| `#autoplay` | Auto-play on load (presence = enabled) | - |
| `#loop` | Loop video (presence = enabled) | - |
| `#mute` | Start muted (presence = enabled) | - |
| `#nocontrols` | Hide player controls (presence = enabled) | - |

### Examples

```sevenmark
// Custom dimensions
[[#youtube #id="dQw4w9WgXcQ" #width="800px" #height="450px"]]

// Custom styling (border, opacity, etc.)
[[#youtube #id="dQw4w9WgXcQ" #style="border-radius:8px;overflow:hidden"]]

// Start at 30 seconds, end at 60 seconds
[[#youtube #id="dQw4w9WgXcQ" #start="30" #end="60"]]

// Autoplay, muted, looping
[[#youtube #id="dQw4w9WgXcQ" #autoplay #mute #loop]]

// No controls (clean embed)
[[#youtube #id="dQw4w9WgXcQ" #nocontrols]]
```

---

## Vimeo

Embed Vimeo videos.

### Basic Video

```sevenmark
[[#vimeo #id="76979871"]]
```

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `#id` | Video ID (required) | - |
| `#h` | Hash for unlisted videos | - |
| `#width` | Player width (CSS value) | `min(640px,100%)` |
| `#height` | Player height (CSS value) | auto via `aspect-ratio:16/9` |
| `#autoplay` | Auto-play on load | - |
| `#loop` | Loop video | - |
| `#mute` | Start muted | - |
| `#color` | Player accent color (hex without #) | - |
| `#dnt` | Do Not Track mode | - |

### Examples

```sevenmark
// Basic embed
[[#vimeo #id="76979871"]]

// Unlisted video with hash
[[#vimeo #id="76979871" #h="abc123def"]]

// Custom color accent
[[#vimeo #id="76979871" #color="ff0000"]]

// Privacy-focused embed
[[#vimeo #id="76979871" #dnt]]
```

---

## NicoNico (nicovideo)

Embed videos from NicoNico Douga (Japanese video platform).

### Basic Video

```sevenmark
[[#nicovideo #id="sm9"]]
```

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `#id` | Video ID (required, e.g., "sm9", "so39402840") | - |
| `#width` | Player width (CSS value) | `min(640px,100%)` |
| `#height` | Player height (CSS value) | auto via `aspect-ratio:16/9` |
| `#from` | Start time in seconds | - |
| `#autoplay` | Auto-play on load | - |

### Examples

```sevenmark
// Classic video
[[#nicovideo #id="sm9"]]

// Start at specific time
[[#nicovideo #id="sm9" #from="60"]]

// Custom size with autoplay
[[#nicovideo #id="sm9" #width="800px" #height="450px" #autoplay]]
```

---

## Spotify

Embed Spotify tracks, albums, playlists, artists, podcasts, and episodes.

### Track

```sevenmark
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC"]]
```

### Album

```sevenmark
[[#spotify #album="4aawyAB9vmqN3uQ7FjRGTy"]]
```

### Playlist

```sevenmark
[[#spotify #playlist="37i9dQZF1DXcBWIGoYBM5M"]]
```

### Artist

```sevenmark
[[#spotify #artist="0OdUWJ0sBjDrqHygGUXeCF"]]
```

### Podcast Episode

```sevenmark
[[#spotify #episode="512ojhOuo1ktJprKbVcKyQ"]]
```

### Podcast Show

```sevenmark
[[#spotify #show="2mTUnDkuKUkhiueKcVWoP0"]]
```

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `#track` | Track ID | - |
| `#album` | Album ID | - |
| `#playlist` | Playlist ID | - |
| `#artist` | Artist ID | - |
| `#episode` | Podcast episode ID | - |
| `#show` | Podcast show ID | - |
| `#width` | Player width (CSS value) | `100%` |
| `#height` | Player height (CSS value) | `352px` |
| `#dark` | Dark theme (presence = enabled) | - |
| `#compact` | Compact cover art view (presence = enabled) | - |

**Note:** One of `#track`, `#album`, `#playlist`, `#artist`, `#episode`, or `#show` is required.

### Examples

```sevenmark
// Dark theme
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC" #dark]]

// Compact view
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC" #compact]]

// Custom dimensions
[[#spotify #playlist="37i9dQZF1DXcBWIGoYBM5M" #width="300px" #height="380px"]]
```

---

## Discord

Embed Discord server widgets.

### Basic Widget

```sevenmark
[[#discord #id="123456789012345678"]]
```

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `#id` | Server ID (required) | - |
| `#width` | Widget width (CSS value) | `min(350px,100%)` |
| `#height` | Widget height (CSS value) | `500px` |
| `#dark` | Dark theme (presence = enabled) | light |

### Examples

```sevenmark
// Light theme (default)
[[#discord #id="123456789012345678"]]

// Dark theme
[[#discord #id="123456789012345678" #dark]]

// Custom dimensions
[[#discord #id="123456789012345678" #width="400px" #height="600px" #dark]]
```

---

## External Media in Complex Structures

### In Lists

```sevenmark
{{{#list #1
[[Video Tutorial:]]
[[[[#youtube #id="dQw4w9WgXcQ"]]]]
[[Music:]]
[[[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC"]]]]
}}}
```

### In Fold Elements

```sevenmark
{{{#fold
[[Watch Video]]
[[
[[#youtube #id="dQw4w9WgXcQ" #width="560px" #height="315px"]]
]]
}}}
```

### Multiple Embeds

```sevenmark
# Media Gallery

## Videos

[[#youtube #id="dQw4w9WgXcQ"]]

[[#vimeo #id="76979871"]]

## Music

[[#spotify #album="4aawyAB9vmqN3uQ7FjRGTy"]]

## Community

[[#discord #id="123456789012345678" #dark]]
```

---

## Best Practices

### Responsive Embeds

All embeds are responsive by default — video embeds use `min(640px,100%)` width with a `16/9` aspect ratio, so they never overflow on mobile without any extra parameters.

To override the default width, use a CSS value:

```sevenmark
// Narrower embed
[[#youtube #id="dQw4w9WgXcQ" #width="400px"]]

// Full container width
[[#youtube #id="dQw4w9WgXcQ" #width="100%"]]
```

### Custom Styling

Use `#style` / `#dark-style` to apply arbitrary CSS to any embed:

```sevenmark
// Rounded corners
[[#youtube #id="dQw4w9WgXcQ" #style="border-radius:12px;overflow:hidden"]]

// Different dark-mode size
[[#spotify #track="4uLU6hMCjMI75M1A2tKUQC" #style="height:152px" #dark-style="height:80px"]]
```

### Privacy Considerations

Use privacy-enhancing options when available:

```sevenmark
// Vimeo Do Not Track
[[#vimeo #id="76979871" #dnt]]

// YouTube without autoplay
[[#youtube #id="dQw4w9WgXcQ"]]
```

### Accessibility

Always provide context around embedded media:

```sevenmark
Watch the introduction video below:

[[#youtube #id="dQw4w9WgXcQ"]]

*Video: Introduction to SevenMark (5 minutes)*
```

## Technical Notes

- External media elements use the `[[#provider ...]]` syntax (double brackets)
- All parameters use the `#key="value"` format
- Boolean parameters (like `#autoplay`, `#mute`, `#dark`) are enabled by their presence
- Missing required parameters will result in an error message being displayed
- Embeds are rendered as iframes with lazy loading enabled
- Each platform has specific CSS classes for styling customization: `sm-embed-youtube`, `sm-embed-vimeo`, `sm-embed-nicovideo`, `sm-embed-spotify`, `sm-embed-discord`
- Default sizing is provided by CSS (responsive by default); `#width` and `#height` accept CSS values (e.g. `800px`, `50%`) and override the defaults
- `#style` and `#dark-style` accept arbitrary inline CSS and are applied via the `data-lk`/`data-dk` shared stylesheet system

</div>
