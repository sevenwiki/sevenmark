# Code Blocks

<div v-pre>

Code blocks display programming code with optional syntax highlighting, using the `{{{#code}}}` syntax.

## Basic Code Block

Display code without language specification:

```sevenmark
{{{#code
function hello() {
    console.log("Hello, World!");
}
}}}
```

## Code with Syntax Highlighting

Specify a programming language using the `#lang` parameter:

```sevenmark
{{{#code #lang="rust"
fn main() {
    println!("Hello, world!");
}
}}}

{{{#code #lang="python"
def greet(name):
    return f"Hello, {name}!"
}}}

{{{#code #lang="javascript"
const greet = (name) => `Hello, ${name}!`;
console.log(greet("World"));
}}}
```

## Supported Languages

SevenMark supports syntax highlighting for many languages:

### Common Languages

- **rust**, **python**, **javascript** / **js**
- **java**, **go**, **cpp** / **c++**, **c**
- **typescript** / **ts**, **csharp** / **cs**

### Web Technologies

- **html**, **css**, **scss**, **sass**
- **json**, **xml**, **yaml** / **yml**
- **markdown** / **md**

### Shell & Scripts

- **bash** / **sh**, **powershell** / **ps1**
- **sql**, **dockerfile**

### Other Languages

- **ruby**, **php**, **swift**, **kotlin**
- **haskell**, **scala**, **elixir**
- And many more!

## Inline Code

Use code blocks inline within text:

```sevenmark
Use the {{{#code console.log() }}} function to output messages.

Call {{{#code #lang="bash" npm install }}} to install dependencies.

The {{{#code #lang="rust" Vec<T> }}} type is a growable array.
```

## Multiline Code Examples

### Rust Example

```sevenmark
{{{#code #lang="rust"
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
}}}
```

### Python Example

```sevenmark
{{{#code #lang="python"
class Calculator:
    def __init__(self):
        self.result = 0

    def add(self, x, y):
        self.result = x + y
        return self.result

    def get_result(self):
        return self.result

calc = Calculator()
print(calc.add(5, 3))  # Output: 8
}}}
```

### JavaScript Example

```sevenmark
{{{#code #lang="javascript"
async function fetchData(url) {
    try {
        const response = await fetch(url);
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
}

fetchData('https://api.example.com/data')
    .then(data => console.log(data))
    .catch(err => console.error(err));
}}}
```

## Code in Complex Structures

### Code in Lists

```sevenmark
{{{#list #1
[[Install Rust: {{{#code #lang="bash" curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh }}}]]
[[Create project: {{{#code #lang="bash" cargo new my_project }}}]]
[[Run project: {{{#code #lang="bash" cargo run }}}]]
}}}
```

### Code in Tables

```sevenmark
{{{#table
[[[[Language]] [[Hello World]]]]
[[[[Rust]] [[{{{#code #lang="rust" println!("Hello!"); }}}]]]]
[[[[Python]] [[{{{#code #lang="python" print("Hello!") }}}]]]]
[[[[JavaScript]] [[{{{#code #lang="javascript" console.log("Hello!"); }}}]]]]
}}}
```

### Code in Folds

```sevenmark
{{{#fold
[[Show Implementation]]
[[
{{{#code #lang="rust"
pub fn parse_document(input: &str) -> Result<Vec<Element>> {
    let mut parser = Parser::new(input);
    parser.parse()
}
}}}
]]
}}}
```

## Configuration Files

### JSON Config

```sevenmark
{{{#code #lang="json"
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0"
  }
}
}}}
```

### YAML Config

```sevenmark
{{{#code #lang="yaml"
server:
  host: localhost
  port: 8080
  ssl:
    enabled: true
    cert: /path/to/cert.pem
}}}
```

### TOML Config

```sevenmark
{{{#code #lang="toml"
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
}}}
```

## Special Characters in Code

Code blocks preserve all characters, including those that would normally be interpreted as SevenMark syntax:

```sevenmark
{{{#code #lang="rust"
// These don't need escaping in code blocks:
let stars = "**not bold**";
let brackets = "[[not a link]]";
let braces = "{{{not a block}}}";
}}}
```

## Styling

Code blocks support parameters for custom styling:

```sevenmark
{{{#code #lang="rust" #style="background: #f5f5f5; border-radius: 5px; padding: 10px"
fn styled_code() {
    println!("Code with custom styling");
}
}}}
```

## Technical Notes

- Code blocks are treated as literal content - markup is not processed inside them
- The `#lang` parameter is case-insensitive: `#lang="Rust"` and `#lang="rust"` are equivalent
- If no language is specified, the code is displayed without syntax highlighting
- Code blocks can be inline (single line) or block (multiple lines)
- Whitespace and indentation are preserved exactly as written
- No escaping is needed inside code blocks

</div>