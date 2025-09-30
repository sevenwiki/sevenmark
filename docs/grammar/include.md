# Include

<div v-pre>

The Include element allows you to embed content from other pages or templates into your current document, enabling content reuse and modular document structures.

## Basic Include

Include content from another page by name:

```sevenmark
{{{#include PageName }}}
{{{#include HeaderTemplate }}}
{{{#include CommonFooter }}}
```

## Include with Parameters

Include elements support optional parameters that can be used to pass data to the included content:

```sevenmark
{{{#include #page="TemplatePage" Content to pass }}}
{{{#include #page="Header" #style="color:blue" Custom styled header }}}
{{{#include #param1="value1" #param2="value2" Template content }}}
```

Parameters can include:
- Standard SevenMark parameters: `#style`, `#size`, `#color`, `#bg_color`, `#opacity`
- Custom parameters for template processing: `#page`, `#topic`, `#author`, etc.

## Include Use Cases

### Template Reuse

```sevenmark
# Article Title

{{{#include #page="ArticleHeader" #author="John Doe" #date="2024-01-15"
Programming Tutorial
}}}

Main article content here...

{{{#include ArticleFooter }}}
```

### Common Sections

```sevenmark
# Documentation Page

{{{#include WarningBanner }}}

## Content

Documentation goes here...

{{{#include RelatedLinks }}}
```

### Parameterized Content

```sevenmark
{{{#include #page="ProductCard" #name="Laptop" #price="$1200" #stock="5"
High-performance laptop for developers
}}}

{{{#include #page="ProductCard" #name="Mouse" #price="$30" #stock="20"
Ergonomic wireless mouse
}}}
```

## Complete Example

```sevenmark
# Programming Tutorial

{{{#include #page="IntroTemplate" #topic="Programming" #level="Beginner"
This tutorial covers fundamental programming concepts.
}}}

## Variables and Data Types

Variables store data that can be used throughout your program...

{{{#include CodeExampleTemplate }}}

## Control Flow

Control flow determines the order in which code executes...

{{{#category Programming }}}
{{{#category Tutorials }}}
```

## Technical Details

### Parser Behavior

- Include elements are processed during the preprocessing stage
- Parameters are parsed as key-value pairs
- Content can contain other SevenMark elements
- Includes are collected and can be resolved by the wiki system

### Parameter Syntax

Parameters follow the standard SevenMark parameter format:
- `#paramName="value"` - String parameter
- Multiple parameters can be specified in any order
- Parameters are optional; simple includes work without them

### Best Practices

1. **Use descriptive page names**: `{{{#include HeaderTemplate }}}` is clearer than `{{{#include Tmpl1 }}}`
2. **Pass necessary context**: Use parameters to make templates flexible
3. **Keep includes focused**: Each included page should serve one clear purpose
4. **Document expected parameters**: If creating templates, document what parameters they expect

## Include vs Other Elements

- **Include**: Embeds content from other pages (supports parameters)
- **Category**: Assigns page to categories (no parameters)
- **Redirect**: Forwards to another page (no parameters)
- **Media**: Embeds images/files using `[[]]` syntax

See also: [Category](category.md), [Redirect](redirect.md), [Media](media.md)

</div>