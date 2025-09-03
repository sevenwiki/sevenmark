# Basic Examples

<div v-pre>

This section demonstrates basic SevenMark usage with simple, practical examples.

## Text Formatting

::: code-group

```sevenmark [Source]
Welcome to **SevenMark**! This is a *powerful* markup language with __many features__.

You can use ~~strikethrough~~ text, add ^^superscript^^ notation for math like E=mc^^2^^,
or use ,,subscript,, for chemical formulas like H,,2,,O.

// This is a comment that won't appear in output
```

```text [Output]
Welcome to SevenMark! This is a powerful markup language with many features.

You can use strikethrough text, add superscript notation for math like E=mc²,
or use subscript for chemical formulas like H₂O.
```

```text [Elements Used]
- **bold**: **text**
- *italic*: *text*
- __underline__: __text__
- ~~strikethrough~~: ~~text~~
- ^^superscript^^: ^^text^^
- ,,subscript,,: ,,text,,
- // comments: // text
```

:::

## Lists

::: code-group

```sevenmark [Numeric List]
{{{#list #1
    [[**Groceries**]]
    [[Milk]]
    [[Bread]]
    [[Eggs]]
    [[Fruits]]
}}}
```

```sevenmark [Alphabetic List]
{{{#list #a
    [[Review documentation]]
    [[Write unit tests]]
    [[Deploy to staging]]
    [[Get approval from team]]
}}}
```

```sevenmark [Roman Numerals]
{{{#list #I
    [[Planning Phase]]
    [[Implementation Phase]]
    [[Testing Phase]]
    [[Deployment Phase]]
}}}
```

```text [List Types]
Available list types:
- #1: Numbers (1, 2, 3...)
- #a: Lowercase (a, b, c...)
- #A: Uppercase (A, B, C...)
- #i: Roman lower (i, ii, iii...)
- #I: Roman upper (I, II, III...)
```

:::

## Tables

::: code-group

```sevenmark [Basic Table]
{{{#table
    [[[[Feature]] [[Basic Plan]] [[Pro Plan]]]]
    [[[[Storage]] [[1GB]] [[100GB]]]]
    [[[[Users]] [[1]] [[Unlimited]]]]
    [[[[Support]] [[Email]] [[24/7 Phone]]]]
}}}
```

```sevenmark [Styled Table]
{{{#table #style="border-collapse: collapse; width: 100%"
    [[[[**Feature**]] [[*Basic*]] [[*Pro*]]]]
    [[[[Storage]] [[1GB]] [[100GB]]]]
    [[[[Users]] [[1 user]] [[~~Unlimited~~ **Unlimited**]]]]
}}}
```

```text [Table Structure]
Tables use nested brackets:
{{{#table
    [[[[Row 1, Col 1]] [[Row 1, Col 2]]]]
    [[[[Row 2, Col 1]] [[Row 2, Col 2]]]]
}}}

- Outer [[ ]]: Table row
- Inner [[ ]]: Table cell
- Can contain any SevenMark elements
```

:::

## Code

::: code-group

```sevenmark [Inline Code]
Use the {{{#code console.log("Hello") }}} function to output messages in JavaScript.

Call {{{#code #lang="bash" npm install }}} to install dependencies.
```

```sevenmark [Code Block]
{{{#code #lang="python"
def greet(name):
    return f"Hello, {name}!"

# Usage example
message = greet("SevenMark")
print(message)
}}}
```

```sevenmark [Multiple Languages]
JavaScript:
{{{#code #lang="javascript"
const greeting = (name) => `Hello, ${name}!`;
console.log(greeting("World"));
}}}

Rust:
{{{#code #lang="rust"
fn main() {
    println!("Hello, World!");
}
}}}
```

```text [Syntax]
Inline: {{{#code your_code_here }}}
Block:  {{{#code #lang="language"
        your_code_here
        }}}

Supported languages:
- javascript, python, rust, go
- html, css, json, yaml
- bash, sql, and many more
```

:::

## Simple Media

### Image with Caption

```sevenmark
[[#file="logo.png" Company Logo]]
```

### Link to Documentation

```sevenmark
For more information, visit [[#url="https://docs.example.com" our documentation]].
```

## Basic Quote

```sevenmark
{{{#quote
"The best way to learn a new markup language is to start with simple examples
and gradually work your way up to more complex structures."
}}}
```

## Math Formula

### Simple Math

```sevenmark
The Pythagorean theorem: {{{#tex a^2 + b^2 = c^2 }}}
```

### Block Math

```sevenmark
{{{#tex #block
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
}}}
```

## Horizontal Divider

```sevenmark
Above the line

---

Below the line
```

</div>
