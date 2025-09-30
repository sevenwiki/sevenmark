# SevenMark Code Style and Conventions

## General Rust Conventions
- **Edition**: Rust 2024
- **Minimum Rust Version**: 1.89.0
- **Formatting**: Standard `rustfmt` (use `cargo fmt`)
- **Linting**: Clippy for additional checks (`cargo clippy`)

## Naming Conventions

### Types and Structs
- **PascalCase** for all types
- Element types end with `Element` suffix: `TextElement`, `TableElement`, `FoldElement`
- Inner/nested elements numbered: `TableInnerElement1`, `TableInnerElement2`
- Traits: `Traversable`, `PreVisitor`

### Functions
- **snake_case** for all functions
- Parser functions typically end with `_parser`: `element_parser`, `table_parser`, `fold_parser`
- Core parsing utilities prefixed: `utils_get_common_style`, `utils_recursion_depth`

### Modules
- **snake_case** for module names
- Often single-word: `brace`, `markdown`, `text`, `token`, `macro`

## Parser Implementation Pattern

All parsers follow this consistent structure:

```rust
pub fn element_parser(input: &mut ParserInput) -> Result<SevenMarkElement> {
    // 1. Capture start position
    let start = input.input.current_token_start();
    
    // 2. Manage recursion depth to prevent stack overflow
    input.state.increase_depth()?;
    
    // 3. Parse using winnow combinators
    let (parameters, content) = delimited(
        literal("start_token"),
        (opt(parameter_core_parser), content_parser),
        literal("end_token")
    ).parse_next(input)?;
    
    // 4. Capture end position
    let end = input.input.previous_token_end();
    
    // 5. Construct AST node with location tracking
    Ok(SevenMarkElement::ElementType(ElementStruct {
        location: Location { start, end },
        common_style: utils_get_common_style(parameters.unwrap_or_default()),
        content
    }))
}
```

## Key Patterns

### Error Handling
- Use `anyhow::Result` for functions that can fail
- Parser functions return `Result<SevenMarkElement>`
- Custom error types in `src/sevenmark/error.rs`

### Context Management
- Always check/increase recursion depth for nested elements
- Use context flags to prevent problematic nesting (e.g., bold inside bold)
- Properly manage state across parser calls

### AST Design
- All elements include `Location { start, end }` for exact position tracking
- Styled elements use `CommonStyleAttributes { style, size, color, bg_color, opacity }`
- All elements implement `Serialize` for JSON output
- Complex elements use `Traversable` trait for visitor pattern

### Visitor Pattern
- Implement `Traversable` for all AST elements
- Visitors use `FnMut(&mut SevenMarkElement)` closures
- Automatic traversal eliminates manual variant handling

## Documentation
- Use doc comments (`///`) for public APIs
- Include examples for complex functions
- Document parser behavior and syntax expectations

## Testing
- Tests organized by element category in `tests/` directory
- Comprehensive integration tests in `comprehensive_parser_tests.rs`
- Test files follow naming: `tests/{category}/test_*.rs`

## Performance Considerations
- Prefer zero-copy parsing where possible
- Use SIMD features from winnow
- Minimize allocations in hot paths
- Track and report parsing performance