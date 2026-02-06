# Redirects

<div v-pre>

Redirects automatically forward users from one page to another.

## Basic Redirect

Redirect to another page:

```sevenmark
{{{#redirect TargetPageName }}}
```

## Redirect with Content

Provide explanatory content alongside the redirect:

```sevenmark
{{{#redirect NewPageName }}}

# This page has moved

The content of this page has been moved to [[NewPageName]]. You will be automatically redirected.
```

## Redirect Behavior

- Redirects should typically be placed at the very beginning of a page
- When a redirect is present, the parser stops processing the rest of the document
- This prevents unnecessary parsing of content that won't be displayed

## Redirect Examples

### Simple Page Move

```sevenmark
{{{#redirect PythonProgramming }}}
```

### Disambiguation

```sevenmark
{{{#redirect Python_(programming_language) }}}

# Python

This page redirects to the programming language. For other uses, see [[Python (disambiguation)]].
```

### Renamed Article

```sevenmark
{{{#redirect ModernWebDevelopment }}}

# Web Development 2.0

This article has been renamed to "Modern Web Development" to better reflect current practices.
```

## Parameters

Since version 2.7.13, redirects support the `#namespace` parameter to redirect to specific namespaces:

```sevenmark
{{{#redirect #namespace="File" image.png }}}
```

### Namespace Examples

```sevenmark
// Redirect to a file page
{{{#redirect #namespace="File" logo.png }}}

// Redirect to a category page
{{{#redirect #namespace="Category" Programming Languages }}}
```

| Parameter | Description |
|-----------|-------------|
| `#namespace` | Target namespace (e.g., `"File"`, `"Category"`) |

When `#namespace` is provided, the redirect target is resolved within the specified namespace.

## Technical Notes

- Only one redirect per page is allowed
- Redirects cannot be nested (A → B → C)
- The redirect target should be a valid page name
- Content after a redirect may not be processed depending on parser configuration
- The `#namespace` parameter can be used to redirect across namespaces
- The entire content inside `{{{#redirect}}}` (excluding parameters) is treated as the target page name and optional message

## Parser Behavior

During preprocessing, redirect elements are collected and can be used to:
- Automatically forward users to the target page
- Update internal links
- Generate redirect index pages
- Track page moves and aliases

</div>