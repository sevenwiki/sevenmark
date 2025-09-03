# Wiki Features

<div v-pre>

SevenMark includes special elements designed for wiki systems.

## Include

Include content from other pages or files:

```sevenmark
{{{#include PageName }}}
{{{#include #page="SomeOtherPage" Custom content }}}
```

### Include with Parameters

```sevenmark
{{{#include #page="TemplatePage" #param1="value1" #param2="value2"
Content to pass to the included page
}}}
```

## Category

Categorize pages:

```sevenmark
{{{#category CategoryName }}}
{{{#category Programming Languages }}}
```

### Multiple Categories

```sevenmark
{{{#category Technology }}}
{{{#category Programming }}}
{{{#category Documentation }}}
```

## Redirect

Redirect to another page (typically used at the beginning of a page):

```sevenmark
{{{#redirect TargetPageName }}}
```

### Redirect with Display Text

```sevenmark
{{{#redirect #target="NewPageName" This page has moved to NewPageName }}}
```

## Media Elements

Embed images and other media:

```sevenmark
[[#file="image.png" Alt text for image]]
[[#url="https://example.com/image.jpg" External image]]
[[#file="document.pdf" #url="backup-url" PDF document]]
```

### Media with Display Text

```sevenmark
[[#file="screenshot.png" Application Screenshot]]
[[#url="https://example.com/video.mp4" Demo Video]]
```

## Complete Wiki Page Example

```sevenmark
{{{#redirect NewArticleName }}}

# This page has been moved

This article has been moved to [[NewArticleName]]. You will be automatically redirected.

{{{#category Redirects }}}
```

Or for a regular wiki page:

```sevenmark
# Programming Tutorial

This tutorial covers basic programming concepts.

## Introduction

{{{#include #page="IntroTemplate" #topic="Programming"
Basic programming introduction
}}}

## Code Example

{{{#code #lang="python"
def hello_world():
    print("Hello, World!")
}}}

## See Also

- [[Advanced Programming]]
- [[Code Style Guide]]

{{{#category Programming }}}
{{{#category Tutorials }}}
```

</div>