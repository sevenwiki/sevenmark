# Categories

<div v-pre>

Categories help organize and classify wiki pages.

## Basic Category Usage

Add a page to a category:

```sevenmark
{{{#category Programming }}}
```

## Multiple Categories

A page can belong to multiple categories:

```sevenmark
{{{#category Technology }}}
{{{#category Programming }}}
{{{#category Documentation }}}
{{{#category Tutorials }}}
```

## Category Names

Category names can contain spaces and special characters:

```sevenmark
{{{#category Programming Languages }}}
{{{#category Web Development }}}
{{{#category C++ Programming }}}
```

## Categories in Context

Categories are typically placed at the end of a page:

```sevenmark
# My Programming Tutorial

This is a tutorial about programming...

## Code Examples

{{{#code #lang="javascript"
console.log("Hello, World!");
}}}

## Conclusion

That concludes our tutorial.

{{{#category Programming }}}
{{{#category JavaScript }}}
{{{#category Tutorials }}}
{{{#category Web Development }}}
```

## Category Organization

Categories help create a hierarchical structure:

```sevenmark
// For a Python tutorial page
{{{#category Programming Languages }}}
{{{#category Python }}}
{{{#category Tutorials }}}

// For a web framework article  
{{{#category Web Development }}}
{{{#category Frameworks }}}
{{{#category Python }}}
{{{#category Django }}}
```

</div>