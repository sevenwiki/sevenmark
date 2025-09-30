# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SevenMark is a Domain Specific Language (DSL) designed for Sevenwiki, implemented in Rust. It's a sophisticated markup parser that handles diverse text formatting elements including text styles, block elements, tables, lists, media elements, and wiki-specific features using advanced parser combinator techniques.

## Development Commands

- `cargo build` - Build the project
- `cargo run` - Run the main application (parser test)
- `cargo test` - Run tests
- `cargo check` - Check code without building
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

## Architecture

The codebase follows a modular parser combinator architecture using winnow with comprehensive context management:

### Core Foundation
- `src/sevenmark/ast.rs` - Abstract Syntax Tree definitions with 50+ comprehensive element types, all serializable with Traversable trait for automatic visitor pattern support
- `src/sevenmark/context.rs` - Parsing context management with recursion depth tracking (max 16 levels)
- `src/sevenmark/error.rs` - Custom error handling types that integrate with winnow
- `src/sevenmark/parser/` - Complete parser implementation organized by functionality
- `src/sevenmark/visitor/` - Visitor pattern implementations including preprocessor with variable substitution

### Parser Input System
- **InputSource**: `LocatingSlice<&str>` for precise position tracking
- **ParserInput**: `Stateful<InputSource, ParseContext>` combining input with parsing state
- **Location Tracking**: All parsed elements include exact start/end positions

### Parser Module Structure
```
parser/
├── document.rs        # Top-level document parsing entry point
├── element.rs         # Main parser dispatcher coordinating all element types
├── brace/            # Brace-enclosed elements {{{#element}}} with parameter support
│   ├── table/        # Table parsing with nested row/cell structure
│   ├── list/         # List parsing with configurable types (1, a, A, i, I)
│   └── ...           # Other brace elements
├── markdown/         # Markdown-style formatting (**, __, ~~, etc.)
├── text/             # Plain text parsing with whitespace handling
├── token/            # Individual symbol parsers for markup tokens
├── macro/            # Macro elements (@age, @now, @tex, etc.)
├── parameter/        # Parameter parsing for complex elements
└── utils/            # Utility functions for common parsing patterns
```

### Context Management Pattern
The parser uses sophisticated context management to prevent infinite recursion and handle nested elements:
- **Recursion Depth Limits**: Configurable maximum depth to prevent stack overflow
- **Element-Specific Context**: Flags to prevent problematic nesting (e.g., bold inside bold)
- **State Preservation**: Context is properly managed across parser calls

### AST Element Hierarchy

**Basic Elements**: Text, Comment, Escape with location tracking

**Styled Content**: CommonStyleAttributes system with style, size, color, bg_color, opacity fields applied consistently across elements

**Block Elements**: 
- TableElement with nested TableInnerElement1/2 structure
- ListElement with configurable list types and ListInnerElement1 items  
- FoldElement for collapsible content sections
- BlockQuoteElement for quoted text blocks

**Media and Wiki Elements**: MediaElement with file/URL parameters, Include/Category/Redirect for wiki functionality

**Markdown Text Styles**: Bold, Italic, BoldItalic, Strikethrough, Underline, Superscript, Subscript, Header with levels

**Advanced Features**: CodeElement with language specification, TeXElement for math, RubyElement for Japanese typography, FootnoteElement, comprehensive macro system

**Variable System**: DefineElement for template variables with `{{{#define #name="value"}}}` syntax, Variable macro `[var(name)]` for substitution, forward-only resolution preventing circular dependencies

### Key Dependencies
- `winnow` (0.7.13 with SIMD features) - Primary parser combinator library
- `serde` (1.0.219 with derive) - AST serialization to JSON
- `anyhow` (1.0.99) - Error handling throughout codebase
- `line-span` (0.1.5) - Efficient line position calculation
- `axum` (0.8.4) - Web framework for future API endpoints
- `tokio` (1.47.1) - Async runtime with full features

### Common Parser Implementation Pattern

When implementing new parsers, follow this established pattern:

```rust
pub fn element_parser(input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = input.input.current_token_start();
    
    // Context management to prevent infinite recursion
    input.state.increase_depth()?;
    
    // Parse using winnow combinators (delimited, repeat, etc.)
    let (parameters, content) = delimited(
        literal("start_token"),
        (opt(parameter_core_parser), content_parser),
        literal("end_token")
    ).parse_next(input)?;
    
    let end = input.input.previous_token_end();
    
    // Create AST node with location tracking
    Ok(SevenMarkElement::ElementType(ElementStruct {
        location: Location { start, end },
        common_style: utils_get_common_style(parameters.unwrap_or_default()),
        content
    }))
}
```

### Visitor Pattern Architecture

The parser includes a sophisticated visitor pattern system for AST processing:

```rust
pub trait Traversable {
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where F: FnMut(&mut SevenMarkElement);
}
```

**Key Features**:
- **Automatic Traversal**: Eliminates need to manually handle all AST variants in visitors
- **Variable Substitution**: 2-stage preprocessor resolves `[var(name)]` references from `{{{#define #name="value"}}}` definitions
- **Forward-only Resolution**: Prevents circular dependencies in variable definitions
- **Preprocessing Pipeline**: Stage 1 substitutes variables, Stage 2 collects metadata (includes, categories, media, redirects)

### Development Workflow
- `ToParse.txt` - Input test file for development
- `ParseResult.json` - Serialized AST output for analysis
- `visualize.py` and `visualization.html` - Parse tree visualization tools
- Built-in performance measurement reporting parsing speed and element counts

The parser uses winnow combinators and follows a recursive descent pattern where complex elements are built from simpler token parsers, with comprehensive error handling and context management throughout.

## Serena MCP Integration

This project is integrated with Serena MCP (Model Context Protocol) for intelligent code analysis and LSP-like semantic understanding. Serena provides symbol-based navigation and editing capabilities.

### Available Serena Capabilities

**Code Analysis:**
- `get_symbols_overview` - View file structure (classes, functions, structs)
- `find_symbol` - Search by name path with depth and filtering (e.g., `TableElement/new`)
- `find_referencing_symbols` - Find all symbol references
- `search_for_pattern` - Flexible regex search with context

**Symbol-Based Editing:**
- `replace_symbol_body` - Replace entire functions/structs/impls
- `insert_after_symbol` / `insert_before_symbol` - Add code at precise locations

**Memory System:**
- Project knowledge stored in `.serena/memories/` (created during onboarding)
- Memories: project overview, codebase structure, commands, style conventions, task completion checklist

### Working with SevenMark Code

**Find Parser Code:**
```
# Find specific parser implementation
find_symbol: name_path="table_parser" relative_path="src/sevenmark/parser/brace/table/"

# Get parser module overview
get_symbols_overview: relative_path="src/sevenmark/parser/brace/mod.rs"
```

**Find AST Elements:**
```
# View all element types
get_symbols_overview: relative_path="src/sevenmark/ast.rs"

# Get specific element with fields
find_symbol: name_path="TableElement" depth=1 include_body=true
```

**Symbol Name Path Syntax:**
- Simple: `method` - matches any symbol named "method"
- Relative: `Class/method` - method in Class at any nesting level
- Absolute: `/Class/method` - top-level Class only

**LSP Symbol Kinds** (for filtering with `include_kinds`/`exclude_kinds`):
- 5=Class, 6=Method, 10=Enum, 11=Trait, 12=Function, 13=Variable, 23=Struct, 26=TypeParameter

### Best Practices with Serena

1. **Start with overview**: Use `get_symbols_overview` before reading full files
2. **Token-efficient**: Only read symbol bodies (`include_body=true`) when needed
3. **Target edits**: Use symbol-based editing instead of line-based for functions/structs
4. **Find impact**: Use `find_referencing_symbols` before making breaking changes