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
- **Variable shadowing**: Later definitions override earlier ones with the same name
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

### Variable References and Document-Order Processing

Since version 2.10.0, variables are processed in **document order** using a single pass. This means a variable defined earlier in the document can be referenced by a later `{{{#define}}}`:

```sevenmark
{{{#define #name="baseUrl" #value="https://example.com"}}}
{{{#define #name="apiUrl" #value="[var(baseUrl)]/api/v1"}}}

API endpoint: [var(apiUrl)]
// Outputs: API endpoint: https://example.com/api/v1
```

#### Conditional Define Pattern

Combine with conditionals to create dynamic variable chains:

```sevenmark
{{{#define #name="env" #value="production"}}}

{{{#if [var(env)] == "production"
{{{#define #name="apiHost" #value="https://api.example.com"}}}
}}}

{{{#if [var(env)] == "development"
{{{#define #name="apiHost" #value="http://localhost:3000"}}}
}}}

Connecting to: [var(apiHost)]
```

### Important Notes

- Variable names should be defined using the `#name` parameter
- Variable values should be defined using the `#value` parameter
- Variables are resolved in document order — a variable can reference any variable defined **before** it
- Circular references are not possible because resolution is forward-only (single pass)
- Undefined variables will produce an error element in the output

</div>