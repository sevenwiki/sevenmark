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
- `src/sevenmark/ast.rs` - Abstract Syntax Tree definitions with 50+ comprehensive element types, all serializable
- `src/sevenmark/context.rs` - Parsing context management with recursion depth tracking (max 16 levels)
- `src/sevenmark/error.rs` - Custom error handling types that integrate with winnow
- `src/sevenmark/parser/` - Complete parser implementation organized by functionality

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

### Development Workflow
- `ToParse.txt` - Input test file for development
- `ParseResult.json` - Serialized AST output for analysis
- `visualize.py` and `visualization.html` - Parse tree visualization tools
- Built-in performance measurement reporting parsing speed and element counts

The parser uses winnow combinators and follows a recursive descent pattern where complex elements are built from simpler token parsers, with comprehensive error handling and context management throughout.