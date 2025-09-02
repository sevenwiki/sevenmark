# SevenMark

A Domain Specific Language (DSL) parser designed for Sevenwiki.

## Overview

SevenMark is a high-performance markup parser that handles diverse text formatting elements including:

- **Text Formatting**: Bold, italic, strikethrough, underline, superscript, subscript
- **Block Elements**: Tables, lists, folds, block quotes, code blocks
- **Media Elements**: Images, files with flexible URL/file parameters
- **Wiki Features**: Includes, categories, redirects, footnotes
- **Advanced Elements**: TeX math expressions, Ruby text, macros
- **Structured Content**: Headers with levels, horizontal rules

## Features

- 🚀 **Fast parsing** with exact location tracking
- 📊 **50+ element types** covering all wiki markup needs
- 🎯 **Extensible architecture** for easy feature additions  
- 📍 **JSON serialization** support for all AST elements
- 🛡️ **Robust error handling** with graceful degradation

### Architecture

```
src/sevenmark/
├── ast.rs              # All element types (FoldElement, TableElement, etc.)
├── context.rs          # Parsing state and recursion depth management  
├── error.rs            # Custom error handling
└── parser/
    ├── element.rs      # Main parser entry point
    ├── brace/         # {{{#fold}}}, {{{#table}}}, {{{#list}}}, etc.
    │   ├── fold/      # Collapsible [[content]] [[blocks]]
    │   ├── table/     # [[[[Cell]]]] structure
    │   └── list/      # Numbered/lettered lists
    ├── markdown/      # **bold**, *italic*, ~~strikethrough~~
    ├── macro/         # [now], @age, @tex macros
    └── utils/         # Common parsing utilities
```

### Key Dependencies

- **winnow** (0.7.13): High-performance parser combinators with SIMD
- **serde** (1.0.219): AST serialization to JSON
- **anyhow** (1.0.99): Comprehensive error handling
- **line-span** (0.1.5): Efficient line position calculation

## Syntax Examples

### Text Formatting
```
**bold** *italic* ~~strikethrough~~ __underline__
^^superscript^^ ,,subscript,,
```

### Lists
```
{{{#list #1
[[First numbered item]]
[[Second numbered item]]
}}}

{{{#list #a
[[First lettered item]]  
[[Second lettered item]]
}}}
```

### Tables
```
{{{#table
[[[[Cell 1]] [[Cell 2]]]]
[[[[Cell 3]] [[Cell 4]]]]
}}}
```

### Folds (Collapsible Content)
```
{{{#fold #style="color:blue"
[[Summary Title]]
[[
Hidden content here
Can contain **any markup**
]]
}}}
```

### Headers
```
# Level 1 Header 
## Level 2 Header 
### Level 3 Header
##! folded header
```

### Code Blocks
```
{{{#code #lang="rust"
fn hello() {
    println!("Hello, SevenMark!");
}
}}}
```

## Performance

SevenMark is designed for high performance:
- SIMD-optimized parsing operations
- Efficient memory management with zero-copy parsing where possible
- Built-in performance measurement and reporting
- Optimized recursion depth management

Typical performance: **>10 MB/s** parsing speed on modern hardware.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Follow the established parser implementation patterns
4. Add tests for new functionality
5. Submit a pull request

---

<div align="center">
<b>Made with ♥️ and lots of ☕ by SevenWiki Team & Community</b>
</div>
