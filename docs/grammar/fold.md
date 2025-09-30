# Fold (Collapsible Sections)

<div v-pre>

Fold elements create collapsible sections with a summary and hidden detail content, similar to HTML's `<details>` and `<summary>` tags.

## Basic Fold

```sevenmark
{{{#fold
[[Summary text]]
[[Hidden detailed content]]
}}}
```

The first `[[]]` block is the summary (always visible), and the second `[[]]` block is the detail content (collapsed by default).

## Fold Structure

Folds use a two-part structure:

1. **Summary** (first `[[]]`): Always visible, clickable to toggle
2. **Detail** (second `[[]]`): Hidden by default, revealed when summary is clicked

```sevenmark
{{{#fold
[[Click to expand]]
[[
This content is hidden until the user clicks the summary.
It can contain multiple lines and any SevenMark elements.
]]
}}}
```

## Styled Folds

Apply styling using parameters:

```sevenmark
{{{#fold #style="border: 1px solid #ccc; padding: 10px"
[[Custom styled fold]]
[[Hidden content with styling]]
}}}

{{{#fold #color="blue" #bg_color="#f0f8ff"
[[Colored fold summary]]
[[Colored detail content]]
}}}
```

## Folds with Rich Content

### Code in Folds

```sevenmark
{{{#fold
[[View Code Example]]
[[
{{{#code #lang="rust"
fn main() {
    println!("Hello from collapsed code!");
}
}}}
]]
}}}
```

### Lists in Folds

```sevenmark
{{{#fold
[[Show Feature List]]
[[
{{{#list #1
[[Feature 1: Fast parsing]]
[[Feature 2: AST serialization]]
[[Feature 3: Extensible architecture]]
}}}
]]
}}}
```

### Tables in Folds

```sevenmark
{{{#fold
[[Show Comparison Table]]
[[
{{{#table
[[[[Feature]] [[Basic]] [[Premium]]]]
[[[[Storage]] [[1GB]] [[100GB]]]]
[[[[Support]] [[Email]] [[24/7]]]]
}}}
]]
}}}
```

## Nested Folds

Folds can be nested inside other folds:

```sevenmark
{{{#fold
[[Level 1: Main Topic]]
[[
Content for main topic.

{{{#fold
[[Level 2: Subtopic]]
[[
Nested content here.

{{{#fold
[[Level 3: Details]]
[[Deeply nested content]]
}}}
]]
}}}
]]
}}}
```

## Common Use Cases

### Documentation Sections

```sevenmark
{{{#fold
[[Advanced Configuration]]
[[
For power users, the following advanced options are available:

{{{#code #lang="yaml"
advanced:
  cache_size: 1000
  optimization_level: 3
  debug_mode: false
}}}
]]
}}}
```

### FAQ Items

```sevenmark
{{{#fold
[[How do I install SevenMark?]]
[[
Installation is simple:

{{{#code #lang="bash"
cargo install sevenmark
}}}

For more details, see the installation guide.
]]
}}}

{{{#fold
[[What languages are supported?]]
[[
SevenMark supports code highlighting for:
- Rust, Python, JavaScript
- Go, Java, C++
- And many more!
]]
}}}
```

### Spoiler Content

```sevenmark
{{{#fold #style="background: #f0f0f0; padding: 5px"
[[=� Spoiler Warning]]
[[
The main character was the villain all along!
]]
}}}
```

## Styling Options

All standard parameters are supported:

- `#style` - Custom CSS styling
- `#color` - Text color
- `#bg_color` - Background color
- `#size` - Font size
- `#opacity` - Opacity level

```sevenmark
{{{#fold #style="border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1)"
[[Beautifully Styled Fold]]
[[Content with custom styling applied]]
}}}
```

## Technical Notes

- Folds require exactly **two `[[]]` blocks**: summary and detail
- The first block is the summary, the second is the detail content
- Detail content can contain any SevenMark elements
- Folds can be nested to any reasonable depth
- Parameters apply to the fold container

</div>