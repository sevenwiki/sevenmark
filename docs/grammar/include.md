# Include

<div v-pre>

The Include element allows you to embed content from other pages or templates into your current document, enabling content reuse and modular document structures.

## Basic Include

Include content from another page by name:

```sevenmark
{{{#include
PageName
}}}

{{{#include
HeaderTemplate
}}}

{{{#include
CommonFooter
}}}
```

## Include with Namespace

Specify the namespace of the included page using the `#namespace` parameter (default: `Document`):

```sevenmark
{{{#include #namespace="Document"
PageName
}}}

{{{#include #namespace="File"
TemplateFile
}}}

{{{#include #namespace="Category"
CategoryPage
}}}
```

## Include with Parameters

Include elements support optional parameters that can be passed to the included content:

```sevenmark
{{{#include #param1="value1" #param2="value2"
TemplatePage
}}}

{{{#include #title="Custom Title" #author="John Doe"
ArticleTemplate
}}}
```

Parameters (excluding `#namespace`) are passed to the included document and can be used with the variable system:
- Any custom parameters you define: `#title`, `#author`, `#date`, etc.
- These parameters override variables defined in the included document

## Include Use Cases

### Template Reuse

```sevenmark
# Article Title

{{{#include #author="John Doe" #date="2024-01-15"
ArticleHeader
}}}

Main article content here...

{{{#include
ArticleFooter
}}}
```

### Common Sections

```sevenmark
# Documentation Page

{{{#include
WarningBanner
}}}

## Content

Documentation goes here...

{{{#include
RelatedLinks
}}}
```

### Parameterized Content

```sevenmark
{{{#include #name="Laptop" #price="$1200" #stock="5"
ProductCard
}}}

{{{#include #name="Mouse" #price="$30" #stock="20"
ProductCard
}}}
```

## Complete Example

```sevenmark
# Programming Tutorial

{{{#include #topic="Programming" #level="Beginner"
IntroTemplate
}}}

## Variables and Data Types

Variables store data that can be used throughout your program...

{{{#include
CodeExampleTemplate
}}}

## Control Flow

Control flow determines the order in which code executes...

{{{#category
Programming
}}}

{{{#category
Tutorials
}}}
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