# Conditionals

<div v-pre>

SevenMark supports conditional rendering using the `{{{#if}}}` element. Content inside the conditional block is included only when the condition evaluates to true.

## Basic Syntax

```sevenmark
{{{#if [var(x)] == "value"
This content is shown when the condition is true.
}}}
```

### With Explicit Delimiter

Use `::` to explicitly separate the condition from content:

```sevenmark
{{{#if [var(count)] > 5 ::
Content with explicit delimiter
}}}
```

## Comparison Operators

| Operator | Description |
|----------|-------------|
| `==` | Equal |
| `!=` | Not equal |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater than or equal |
| `<=` | Less than or equal |

### Examples

```sevenmark
{{{#if [var(count)] == 10
Count is exactly 10
}}}

{{{#if [var(score)] >= 60
Passed!
}}}

{{{#if [var(level)] < 5
Beginner level
}}}
```

## Logical Operators

| Operator | Description |
|----------|-------------|
| `&&` | AND |
| `\|\|` | OR |
| `!` | NOT (prefix) |

### Examples

```sevenmark
{{{#if [var(loggedIn)] == "true" && [var(isAdmin)] == "true"
Admin dashboard
}}}

{{{#if [var(premium)] == "true" || [var(trial)] == "true"
Access granted
}}}

{{{#if ![var(banned)] == "true"
User is not banned
}}}
```

## Grouping with Parentheses

Use parentheses to control operator precedence:

```sevenmark
{{{#if ([var(a)] == "1" || [var(b)] == "2") && [var(c)] != null
Complex condition with grouping
}}}
```

## Type Conversion Functions

| Function | Description |
|----------|-------------|
| `int(expr)` | Convert to integer |
| `len(expr)` | Get string length |
| `str(expr)` | Convert to string |

### Examples

```sevenmark
{{{#if int([var(age)]) >= 18
Adult content
}}}

{{{#if len([var(name)]) > 0
Name is not empty
}}}
```

## Null Checks

Check if a variable is defined or undefined:

```sevenmark
{{{#if [var(optional)] == null
Variable is not defined
}}}

{{{#if [var(required)] != null
Variable is defined
}}}
```

### Null Guard Pattern

Use short-circuit evaluation for safe access:

```sevenmark
{{{#if [var(x)] != null && int([var(x)]) > 5
x is defined and greater than 5
}}}
```

If `x` is null, the right side of `&&` is not evaluated (short-circuit).

## Boolean Literals

Use `true` and `false` keywords:

```sevenmark
{{{#if [var(enabled)] == true
Feature is enabled
}}}

{{{#if [var(disabled)] == false
Feature is not disabled
}}}

{{{#if (5 > 3) == true
Condition result comparison
}}}
```

## Type Coercion Rules

### Equality (`==`, `!=`)

- `"5" == 5` is `true` (string parsed as number)
- `"abc" == 5` is `false` (cannot parse)
- `null == null` is `true`
- `true == true` is `true`

### Numeric Comparison (`>`, `<`, `>=`, `<=`)

Both values must be convertible to numbers:

- `10 > 5` is `true`
- `"10" > 5` is `true` (string parsed as number)
- `"abc" > 5` is `false` (cannot compare, not converted to 0)
- `null > 5` is `false` (null is not comparable)

## Complex Examples

### Conditional Navigation

```sevenmark
{{{#define #role="admin"}}}

{{{#if [var(role)] == "admin"
**Admin Menu**
- User Management
- System Settings
- Analytics Dashboard
}}}

{{{#if [var(role)] == "user"
**User Menu**
- My Profile
- My Orders
}}}
```

### Conditional Formatting

```sevenmark
{{{#define #score="85"}}}

{{{#if int([var(score)]) >= 90
{{{ #style="color:green" **Excellent!** Score: [var(score)] }}}
}}}

{{{#if int([var(score)]) >= 60 && int([var(score)]) < 90
{{{ #style="color:blue" **Good!** Score: [var(score)] }}}
}}}

{{{#if int([var(score)]) < 60
{{{ #style="color:red" **Needs Improvement.** Score: [var(score)] }}}
}}}
```

### With Tables (Block Level)

Conditionals can wrap entire table rows as content blocks:

```sevenmark
{{{#define #showDetails="true"}}}

{{{#table
[[[[Product]] [[Price]]]]
[[[[Widget A]] [[$10]]]]
{{{#if [var(showDetails)] == "true"
[[[[Widget A Details]] [[Size: Medium, Color: Blue]]]]
}}}
[[[[Widget B]] [[$20]]]]
}}}
```

### Table Row Conditionals

For more precise control, use conditionals at the row level inside tables:

```sevenmark
{{{#table
[[[[Header 1]] [[Header 2]]]]
[[[[Normal Row]] [[Data]]]]
{{{#if [var(condition)] == "true" :: [[[[Conditional Row]] [[More Data]]]] }}}
[[[[Footer]] [[End]]]]
}}}
```

Key difference: Row-level conditionals use `::` delimiter and contain row syntax `[[[[cell]] [[cell]]]]` directly.

### Table Cell Conditionals

Conditionals can also control individual cells within a row:

```sevenmark
{{{#table
[[ [[Product]] [[Price]] {{{#if [var(showStock)] == "true" :: [[Stock]] }}} ]]
[[ [[Widget]] [[$10]] {{{#if [var(showStock)] == "true" :: [[5 units]] }}} ]]
}}}
```

### List Item Conditionals

Similar syntax works for list items:

```sevenmark
{{{#list #1
[[Always visible item]]
{{{#if [var(showExtra)] == "true" :: [[Conditional item 1]] [[Conditional item 2]] }}}
[[Another visible item]]
}}}
```

Multiple items can be included in a single conditional block.

## Processing Order

1. Variables are substituted first (`{{{#define}}}` → `[var()]`)
2. Conditions are evaluated
3. Content is expanded (if true) or removed (if false)
4. Nested elements inside conditionals are processed normally

## Truthy/Falsy Evaluation

When an expression is used as a boolean (e.g., in logical operators), SevenMark follows JavaScript-style truthy/falsy rules:

### Falsy Values

| Value | Evaluates To |
|-------|--------------|
| `null` | `false` |
| `""` (empty string) | `false` |
| `0` | `false` |
| `false` | `false` |

### Truthy Values

| Value | Evaluates To |
|-------|--------------|
| `"0"` | `true` (non-empty string) |
| `"false"` | `true` (non-empty string) |
| Any non-empty string | `true` |
| Non-zero numbers | `true` |
| `true` | `true` |

### Important: String "0" and "false" are Truthy

Unlike some languages, the strings `"0"` and `"false"` evaluate to **true** because they are non-empty strings:

```sevenmark
{{{#define #value="0"}}}

{{{#if [var(value)]
This WILL be shown because "0" is a non-empty string (truthy)
}}}
```

### Checking String Values Properly

To check if a string represents a falsy concept, use explicit comparison:

```sevenmark
{{{#define #enabled="false"}}}

// Wrong: "false" is truthy (non-empty string)
{{{#if [var(enabled)]
This will be shown!
}}}

// Correct: Use explicit comparison
{{{#if [var(enabled)] == "true"
This will NOT be shown
}}}

// Also correct: Convert to int for numeric checks
{{{#define #count="0"}}}
{{{#if int([var(count)]) != 0
This will NOT be shown (int("0") == 0)
}}}
```

### Logical Operator Evaluation

```sevenmark
// Truthy evaluation in && and ||
{{{#if [var(name)] && [var(email)]
Both name and email are non-empty
}}}

// NOT operator
{{{#if ![var(disabled)]
Disabled is null or empty
}}}

// Double negation (use !(!x) syntax)
{{{#if !(!([var(value)]))
Value is truthy
}}}
```

## String Literals

String literals are enclosed in double quotes. Since version 2.21.4, escape sequences are supported inside string literals:

| Escape Sequence | Result |
|-----------------|--------|
| `\"` | Literal double quote |
| `\\` | Literal backslash |

### Examples

```sevenmark
{{{#if [var(title)] == "He said \"hello\""
Match a string containing double quotes
}}}

{{{#if [var(path)] == "C:\\Users\\docs"
Match a string containing backslashes
}}}

{{{#if [var(msg)] == "line1\\line2 and \"quoted\""
Combined escape sequences
}}}
```

Without escape sequences, it is impossible to compare against strings that contain double quotes or backslashes.

## Important Notes

- Conditions are resolved during preprocessing phase
- Supports nested formatting inside conditional blocks
- Short-circuit evaluation prevents unnecessary computation
- Undefined variables evaluate to `null`
- Empty strings are falsy, **non-empty strings (including "0" and "false") are truthy**
- For strict boolean checks, use `== "true"` or `== "false"` comparisons
- Only single `!` operator is allowed; use `!(!x)` for double negation
- String literals support `\"` and `\\` escape sequences

</div>
