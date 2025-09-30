# TeX Math

<div v-pre>

TeX elements allow you to include mathematical expressions and formulas using LaTeX syntax.

## Inline Math

Display mathematical expressions inline with text:

```sevenmark
The famous equation {{{#tex E = mc^2 }}} was discovered by Einstein.

The quadratic formula is {{{#tex x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} }}}.
```

## Block Math

Use `#block` parameter for display-style math that appears on its own line:

```sevenmark
{{{#tex #block
E = mc^2
}}}

{{{#tex #block
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
}}}
```

## Common Mathematical Notation

### Basic Operations

```sevenmark
Addition: {{{#tex a + b }}}
Subtraction: {{{#tex a - b }}}
Multiplication: {{{#tex a \times b }}} or {{{#tex a \cdot b }}}
Division: {{{#tex \frac{a}{b} }}}
```

### Exponents and Indices

```sevenmark
Superscript: {{{#tex x^2 }}} or {{{#tex x^{10} }}}
Subscript: {{{#tex x_i }}} or {{{#tex x_{ij} }}}
Combined: {{{#tex x_i^2 }}}
```

### Greek Letters

```sevenmark
Lowercase: {{{#tex \alpha, \beta, \gamma, \delta, \theta, \pi, \sigma }}}
Uppercase: {{{#tex \Delta, \Gamma, \Lambda, \Sigma, \Omega }}}
```

### Fractions and Roots

```sevenmark
Fraction: {{{#tex \frac{numerator}{denominator} }}}
Square root: {{{#tex \sqrt{x} }}}
N-th root: {{{#tex \sqrt[n]{x} }}}
```

## Advanced Formulas

### Summation and Products

```sevenmark
{{{#tex #block
\sum_{i=1}^{n} x_i = x_1 + x_2 + \cdots + x_n
}}}

{{{#tex #block
\prod_{i=1}^{n} x_i = x_1 \times x_2 \times \cdots \times x_n
}}}
```

### Integrals

```sevenmark
{{{#tex #block
\int_{a}^{b} f(x) \, dx
}}}

{{{#tex #block
\int_{-\infty}^{\infty} e^{-x^2} \, dx = \sqrt{\pi}
}}}
```

### Limits

```sevenmark
{{{#tex #block
\lim_{x \to \infty} \frac{1}{x} = 0
}}}

{{{#tex #block
\lim_{h \to 0} \frac{f(x+h) - f(x)}{h} = f'(x)
}}}
```

### Matrices

```sevenmark
{{{#tex #block
\begin{bmatrix}
a & b \\
c & d
\end{bmatrix}
}}}

{{{#tex #block
\begin{pmatrix}
1 & 2 & 3 \\
4 & 5 & 6 \\
7 & 8 & 9
\end{pmatrix}
}}}
```

## Mathematical Environments

### Equations

```sevenmark
{{{#tex #block
ax^2 + bx + c = 0
}}}

{{{#tex #block
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
}}}
```

### Systems of Equations

```sevenmark
{{{#tex #block
\begin{cases}
x + y = 5 \\
2x - y = 1
\end{cases}
}}}
```

### Aligned Equations

```sevenmark
{{{#tex #block
\begin{align*}
f(x) &= x^2 + 2x + 1 \\
&= (x + 1)^2
\end{align*}
}}}
```

## Special Symbols and Operators

### Set Theory

```sevenmark
{{{#tex A \cup B }}} (union)
{{{#tex A \cap B }}} (intersection)
{{{#tex A \subseteq B }}} (subset)
{{{#tex x \in A }}} (element of)
{{{#tex \emptyset }}} (empty set)
```

### Logic

```sevenmark
{{{#tex \land }}} (and)
{{{#tex \lor }}} (or)
{{{#tex \neg }}} (not)
{{{#tex \forall }}} (for all)
{{{#tex \exists }}} (exists)
```

### Relations

```sevenmark
{{{#tex \leq }}} (less than or equal)
{{{#tex \geq }}} (greater than or equal)
{{{#tex \neq }}} (not equal)
{{{#tex \approx }}} (approximately)
{{{#tex \equiv }}} (equivalent)
```

## TeX in Complex Structures

### Math in Lists

```sevenmark
{{{#list #1
[[The Pythagorean theorem: {{{#tex a^2 + b^2 = c^2 }}}]]
[[Euler's identity: {{{#tex e^{i\pi} + 1 = 0 }}}]]
[[The golden ratio: {{{#tex \phi = \frac{1 + \sqrt{5}}{2} }}}]]
}}}
```

### Math in Tables

```sevenmark
{{{#table
[[[[Formula]] [[Description]]]]
[[[[{{{#tex E = mc^2 }}}]] [[Mass-energy equivalence]]]]
[[[[{{{#tex F = ma }}}]] [[Newton's second law]]]]
[[[[{{{#tex a^2 + b^2 = c^2 }}}]] [[Pythagorean theorem]]]]
}}}
```

### Math in Folds

```sevenmark
{{{#fold
[[Show Proof]]
[[
{{{#tex #block
\begin{align*}
(a + b)^2 &= (a + b)(a + b) \\
&= a^2 + ab + ba + b^2 \\
&= a^2 + 2ab + b^2
\end{align*}
}}}
]]
}}}
```

## Common Formulas

### Physics

```sevenmark
Newton's second law: {{{#tex F = ma }}}
Kinetic energy: {{{#tex KE = \frac{1}{2}mv^2 }}}
Gravitational force: {{{#tex F = G\frac{m_1 m_2}{r^2} }}}
```

### Calculus

```sevenmark
Derivative: {{{#tex \frac{df}{dx} }}} or {{{#tex f'(x) }}}
Integral: {{{#tex \int f(x) \, dx }}}
Chain rule: {{{#tex \frac{d}{dx}f(g(x)) = f'(g(x)) \cdot g'(x) }}}
```

### Statistics

```sevenmark
Mean: {{{#tex \mu = \frac{1}{n}\sum_{i=1}^{n} x_i }}}
Standard deviation: {{{#tex \sigma = \sqrt{\frac{1}{n}\sum_{i=1}^{n}(x_i - \mu)^2} }}}
Normal distribution: {{{#tex f(x) = \frac{1}{\sigma\sqrt{2\pi}}e^{-\frac{1}{2}(\frac{x-\mu}{\sigma})^2} }}}
```

## Technical Notes

- TeX elements use standard LaTeX math syntax
- The `#block` parameter marks math for display-style presentation
- Without `#block`, math is marked for inline presentation
- Complex expressions may require proper grouping with braces `{}`
- Most standard LaTeX math commands are supported in the syntax
- The parser preserves the LaTeX content as-is within the AST

</div>