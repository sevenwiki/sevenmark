# Macros

<div v-pre>

SevenMark supports various macro elements for dynamic content generation.

## Time Macros

### Current Time

Display the current timestamp:

```sevenmark
Current time: [now]
```

### Age Calculation

Calculate age from a given date:

```sevenmark
Age calculation: [age(1990-01-15)]
Born in 1995: [age(1995-06-20)]
```

## Utility Macros

### Line Break

Insert a line break:

```sevenmark
Line break:[br]Next line
```

### Null Macro

The null macro produces no output:

```sevenmark
This text[null]continues without interruption.
```

## Macro Usage in Complex Elements

Macros can be used within other elements:

```sevenmark
{{{#list #1
[[Item with current time: [now]]]
[[Person born in 1990 is [age(1990-01-01)] years old]]
[[Line 1[br]Line 2 in same item]]
}}}
```

### In Tables

```sevenmark
{{{#table
[[[[Name]] [[Age]]]]
[[[[John]] [[[age(1985-03-15)]]]]]
[[[[Updated]] [[[now]]]]]
}}}
```

### In Styled Text

```sevenmark
{{{ #style="color:blue" Last updated: [now] }}}
```

## Variable System

### Define Variables

Create template variables using `{{{#define}}}`:

```sevenmark
{{{#define #name="projectName" #value="SevenMark"}}}
{{{#define #name="version" #value="2.0"}}}
{{{#define #name="author" #value="SevenWiki Team"}}}
```

### Variable Substitution

Reference defined variables using the `[var()]` macro:

```sevenmark
Welcome to [var(projectName)] version [var(version)]!
Created by [var(author)].
```

### Variable Usage Examples

#### Document Header Template

```sevenmark
{{{#define #name="docTitle" #value="API Reference"}}}
{{{#define #name="docVersion" #value="v1.2.3"}}}
{{{#define #name="lastUpdate" #value="2024-01-15"}}}

# [var(docTitle)] - [var(docVersion)]

Last updated: [var(lastUpdate)]
```

#### Repeated Content

```sevenmark
{{{#define #name="companyName" #value="Acme Corporation"}}}
{{{#define #name="supportEmail" #value="support@acme.com"}}}

Welcome to [var(companyName)]!

For assistance, contact [var(companyName)] support at [var(supportEmail)].

© 2024 [var(companyName)]. All rights reserved.
```

### Variable Scope and Resolution

- Variables are resolved in a **forward-only** manner to prevent circular dependencies
- Variables must be defined before they are used
- Variable substitution occurs during the preprocessing stage
- Variables can be used in any SevenMark element after definition

```sevenmark
{{{#define #name="greeting" #value="Hello"}}}

# [var(greeting)], World!

{{{#list #1
[[First item uses: [var(greeting)]]]
[[Second item also uses: [var(greeting)]]]
}}}
```

### Important Notes

- Variable names should be defined using the `#name` parameter
- Variable values should be defined using the `#value` parameter
- Variables cannot reference other variables (no nested variable expansion)
- Undefined variables will remain as `[var(name)]` in the output

</div>