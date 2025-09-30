# SevenMark Codebase Structure

## Directory Layout

```
sevenmark/
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── main.rs                   # Server binary entry point (sevenmark-server)
│   ├── state.rs                  # Application state management
│   ├── api/                      # REST API implementation (v0)
│   ├── config/                   # Configuration management
│   ├── connection/               # Database connections
│   ├── errors/                   # Error types
│   ├── utils/                    # Utility functions
│   ├── bin/                      # Binary executables
│   │   ├── parse.rs             # Parse without preprocessing
│   │   ├── svm_file.rs          # Parse with preprocessing (includes, redirects, etc.)
│   │   ├── monaco.rs            # Monaco editor integration
│   │   ├── debug_*.rs           # Debug utilities
│   └── sevenmark/               # Core parser implementation
│       ├── ast.rs               # 50+ AST element type definitions
│       ├── context.rs           # Parsing context with recursion depth tracking
│       ├── error.rs             # Custom error types
│       ├── core.rs              # Core parsing functions
│       ├── parser/              # Parser combinator implementations
│       │   ├── document.rs     # Top-level document parser
│       │   ├── element.rs      # Main parser dispatcher
│       │   ├── brace/          # {{{#element}}} parsers
│       │   │   ├── brace_*.rs  # Individual brace element parsers
│       │   │   ├── table/      # Table parsing
│       │   │   ├── list/       # List parsing
│       │   │   ├── fold/       # Fold/collapsible content
│       │   │   ├── code/       # Code blocks
│       │   │   ├── literal/    # Literal content
│       │   │   ├── category/   # Wiki categories
│       │   │   ├── include/    # Wiki includes
│       │   │   └── redirect/   # Wiki redirects
│       │   ├── markdown/       # Markdown-style formatting (**, __, ~~)
│       │   ├── text/           # Plain text parsing
│       │   ├── token/          # Token parsers
│       │   ├── macro/          # Macro elements (@age, @now, @tex)
│       │   ├── parameter/      # Parameter parsing
│       │   ├── bracket/        # Bracket elements (media)
│       │   ├── comment/        # Comment parsing (inline/multiline)
│       │   └── utils/          # Parser utility functions
│       └── visitor/            # Visitor pattern implementations
│           ├── preprocessor.rs # 2-stage preprocessor (variables + metadata)
│           └── monaco.rs       # Monaco editor output
├── tests/                      # Integration tests
│   ├── comprehensive_parser_tests.rs
│   ├── monaco_tests.rs
│   └── [category]/            # Tests organized by element type
├── docs/                       # Documentation
├── ToParse.txt                # Development test input
├── ParseResult.json           # AST output
├── PreprocessInfo.json        # Preprocessing metadata
└── visualization.html         # Parse tree visualizer
```

## Key Modules

### Core Parser (`src/sevenmark/`)
- **ast.rs**: Defines 50+ element types (TextElement, TableElement, ListElement, etc.) with Traversable trait
- **context.rs**: ParseContext with recursion depth management (max 16 levels)
- **core.rs**: Entry points `parse_document()` and `parse_document_with_preprocessing()`

### Parser Input System
- **InputSource**: `LocatingSlice<&str>` for position tracking
- **ParserInput**: `Stateful<InputSource, ParseContext>` combines input with state
- All elements track exact `Location { start, end }` positions

### Visitor Pattern
- **Traversable trait**: Enables automatic AST traversal without manual variant handling
- **PreVisitor**: 2-stage preprocessing (Stage 1: variable substitution, Stage 2: metadata collection)
- **SevenMarkPreprocessor**: Collects includes, categories, media, redirects