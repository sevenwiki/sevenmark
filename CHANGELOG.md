# Changelog

All notable changes to SevenMark parser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.19] - 2025-09-29

### Added
- **DefineElement Support**: New AST element for variable definition and substitution
  - Added `DefineElement` for `{{{#define #name="value"}}}` syntax parsing
  - Enables variable definition that can be referenced with `[var(name)]` macro
  - Struct includes location tracking and parameter storage for defined variables
  - Integrated into main `SevenMarkElement` enum for full parser support

### Improved
- **Visitor Pattern Architecture**: Redesigned AST traversal with trait-based approach
  - Added `Traversable` trait for automatic AST element traversal
  - Reduced preprocessor code complexity by 50% (180 → 87 lines)
  - Eliminated need to manually handle all AST variants in visitors
  - New elements automatically supported by implementing trait pattern
  - Enhanced maintainability and reduced code duplication across visitor implementations

## [2.0.16] - 2025-09-07

### Added
- **MediaElement Context Protection**: Added `inside_media_element` context flag to prevent nested MediaElement parsing
  - MediaElement (`[[content]]`) now prevents infinite nesting for better parsing stability
  - Uses same pattern as footnote context management for consistency

### Improved
- **Parser Code Quality**: Refactored all markdown parsers to use `with_depth` utility
  - Simplified markdown_bold, markdown_italic, markdown_strikethrough, markdown_underline parsers
  - Refactored markdown_superscript, markdown_subscript, and markdown_header parsers
  - Eliminated complex manual depth management in favor of centralized utility function
  - Improved code consistency and maintainability across all text formatting parsers

## [2.0.15] - 2025-09-07

### Added
- **New Parser Elements**: Added support for footnote, ruby, and variable elements
  - `{{{#fn content}}}` - Footnote parser with nested content support and anti-recursion protection
  - `{{{#ruby #ruby="text" content}}}` - Ruby text parser for Japanese typography with parameter support
  - `[var(name)]` - Variable macro parser for template variable substitution
  - `[age(YYYY-MM-DD)]` - Enhanced age macro parser with proper location tracking

### Enhanced
- **AST Structure Improvements**: Unified parameter handling across elements
  - Added `Parameters` field to `RubyElement` and `CodeElement` for consistency
  - Converted `Age` and `Variable` from simple enum variants to structured elements with location tracking
  - All brace elements now follow consistent parameter pattern for visitor processing

### Improved
- **Parser Context Management**: Enhanced recursion and nesting prevention
  - Added `inside_footnote` context flag to prevent infinite footnote nesting
  - Footnote parser follows established markdown parser patterns for context handling
  - Improved error handling with proper context preservation

- **Visitor Pattern**: Comprehensive AST traversal support
  - Updated preprocessor visitor with complete element coverage
  - Added explicit handling for all 43 AST element types with clear categorization
  - Enhanced documentation for visitor extension and maintenance

## [2.0.14] - 2025-09-06

### Changed
- **Parameter Storage Optimization**: Replaced HashMap with BTreeMap for parameter storage
  - Ensures consistent parameter ordering across multiple parsing sessions
  - Improves test stability by eliminating non-deterministic hash-based ordering
  - Parameters now appear in alphabetical key order in serialized output

### Performance
- **Monaco Editor Memory Optimization**: Reduced memory usage from O(bytes) to O(lines)
  - Replaced byte-level position mapping with line-based binary search approach
  - Memory usage now scales with document line count instead of total bytes
  - Maintained efficient O(log n) position lookup performance using `line_span` crate
  - Significant memory savings for large documents (e.g., 1MB document: 2MB → ~1KB memory)

### Added
- **Monaco Editor Test Suite**: Comprehensive test coverage for position conversion
  - Added `tests/monaco/` directory with input/expected file pairs
  - Tests for UTF-8 handling, complex elements, markdown formatting, and edge cases
  - Automated verification of byte-to-line/column position accuracy

## [2.0.12] - 2025-09-05

### Added
- **Monaco Editor Support**: Complete integration for Monaco Editor decorations
  - New `parse_sevenmark_to_monaco` WASM function for web integration
  - O(1) byte-to-line/column position conversion with pre-computed mapping arrays
  - `MonacoVisitor` for converting AST elements to Monaco-compatible JSON format
  - `LineColumnLocation` struct with 1-based line/column positions
  - Efficient handling of UTF-8 multi-byte characters in position calculations

### Changed
- **Location System Enhancement**: AST Location now supports JSON deserialization
  - Added `Deserialize` trait to Location struct for JSON processing
  - Enables round-trip serialization/deserialization of parsed documents
  - Improved compatibility with web-based development tools

## [2.0.11] - 2025-09-04

### Added
- **Parameter Location Tracking**: Parameters now include precise location information
  - Each parameter contains `location`, `key`, and `value` fields
  - Enables accurate source position tracking for debugging and development tools
  - Better error reporting with exact parameter positions

### Changed  
- **Parser Architecture Refactoring**: Separated parsing from semantic analysis
  - Parsers now store raw `parameters` instead of processed `CommonStyleAttributes`
  - Semantic processing moved to visitor pattern for better separation of concerns
  - Maintains all location information throughout parsing pipeline

## [2.0.10] - 2025-09-04

### Added
- **WASM Parser Build**: Enhanced WebAssembly compilation support
  - Optimized for web browser integration
  - Lightweight binary output for client-side parsing
  - Enables future syntax highlighting and editor integration capabilities

### Changed  
- **Performance Improvements**: Comprehensive parser optimization
  - Enhanced parser combinator usage patterns
  - Reduced memory allocations during parsing
  - Improved parsing speed for complex nested structures
  - Better context management with depth limiting

---

## Previous Versions

For earlier versions, please refer to git history or previous documentation.