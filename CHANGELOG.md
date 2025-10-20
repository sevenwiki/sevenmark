# Changelog

All notable changes to SevenMark parser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.4.0] - 2025-10-21
### Changed
- **Database Access**: Replaced HTTP API calls with direct database access
  - Removed `WikiClient` and HTTP-based document fetching
  - Implemented direct PostgreSQL queries using Sea ORM
  - Added `bridge` module for database operations (`fetch_documents_batch`)
  - Significantly improved performance by eliminating network overhead
  - Removed unnecessary serialization/deserialization steps

### Added
- **Entity Structure**: Created organized entity modules for database tables
  - Added `entity/document_metadata.rs` for document metadata table
  - Added `entity/document_revisions.rs` for document revisions table
  - Added `entity/document_files.rs` for document files table
  - Type-safe entity aliases: `DocumentMetadata`, `DocumentMetadataColumn`, etc.
  - Proper Sea ORM entity definitions with derives and relations

- **Type System**: Enhanced `DocumentNamespace` with Sea ORM support
  - Added `DeriveActiveEnum` and `EnumIter` derives to `DocumentNamespace`
  - Direct mapping to PostgreSQL enum: `document`, `file`, `category`
  - Unified type for both business logic and database operations

### Removed
- **Deprecated HTTP Layer**: Cleaned up unused HTTP API components
  - Removed `client.rs` module (WikiClient implementation)
  - Removed `GetDocumentRequest` type (HTTP-only)
  - Removed `GetDocumentsBatchRequest` type (HTTP-only)
  - Removed `DocumentListResponse` type (HTTP-only)
  - Simplified `types.rs` to essential types only

### Configuration
- **Database Configuration**: Added PostgreSQL connection settings
  - New environment variables: `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_HOST`, `POSTGRES_PORT`, `POSTGRES_NAME`
  - Connection pool configuration: `POSTGRES_MAX_CONNECTION`, `POSTGRES_MIN_CONNECTION`
  - Removed: `WIKI_SERVER_HOST`, `WIKI_SERVER_PORT` (no longer needed)

## [2.3.0] - 2025-10-04
### Added
- **REST API Endpoints**: New document parsing HTTP endpoint
  - Added `POST /v0/parse` endpoint for document parsing and processing
  - Accepts `ParseDocumentRequest` with raw SevenMark content
  - Returns `ProcessedDocument` with resolved AST, categories, and redirect metadata
  - Full OpenAPI/Swagger documentation with `ParseApiDoc`
  - Integrated with WikiClient for automatic media and include resolution

### Changed
- **Application State**: Refactored state management for better architecture
  - Replaced `http_client` with `WikiClient` in `AppState`
  - WikiClient now initialized at startup with configured wiki server URL from environment
  - Centralized wiki backend configuration through `DbConfig` (WIKI_SERVER_HOST, WIKI_SERVER_PORT)
  - Improved separation of concerns between HTTP transport and wiki-specific logic

## [2.2.4] - 2025-10-04
### Added
- **Media Resolution System**: Complete postprocessing pipeline for media references
  - Added `ResolvedMediaInfo` to `MediaElement` for storing resolved URLs and validation status
  - Created `postprocessor` module for resolving media references to actual URLs
  - File namespace references resolve to storage URLs via wiki API
  - Document/Category namespace references generate proper page links (`/document/{title}`, `/category/{title}`)
  - URL parameters pass through without additional processing
  - Invalid references marked with `is_valid: false` for error handling

- **Processing Pipeline**: Unified document processing with pre/post stages
  - Created `processor` module combining preprocessing and postprocessing
  - `process_sevenmark()` function orchestrates full pipeline automatically
  - Seamless integration of include resolution and media resolution
  - Single entry point for complete document transformation

- **Type System Enhancements**: Improved type safety for media handling
  - Added `MediaReference` type for tracking namespace+title pairs
  - `PreProcessedDocument` now uses structured `MediaReference` instead of plain strings
  - Better type safety for media collection and resolution workflow


### Changed
- **WikiClient Enhancement**: Added comprehensive debug logging
  - Request logging shows each document being fetched with namespace:title format
  - Response logging displays received documents and file_url presence
  - Better visibility into wiki API interactions for debugging

- **API Response Structure**: Fixed `DocumentResponse` to match backend schema
  - Moved `file_url` from `DocumentRevision` to top-level `DocumentResponse`
  - Corrected deserialization to handle actual API response format
  - Aligned type definitions with backend API specification

### Fixed
- **Media Collection**: Preprocessor now correctly collects file/document/category references
  - Fixed to use `MediaReference` with proper namespace tracking
  - URL parameters no longer incorrectly added to fetch list
  - Only file/document/category parameters trigger wiki API requests

## [2.2.0] - 2025-10-04

### Changed
- **Include Processing Simplification**: Simplified to 1-depth include resolution
  - Removed recursive include processing (includes now resolve only one level deep)
  - Preprocessor now accepts parsed AST instead of raw string input for better modularity
  - Improved memory efficiency by removing unnecessary Vec allocations and clones
  - Each document fetched only once, then cloned per parameter combination

### Removed
- Maximum depth limiting (no longer needed with 1-depth includes)
- Circular reference detection system (no longer needed)
- `async-recursion, blake3` dependency
- `visited` HashSet tracking

### Fixed
- Duplicate document fetching when same document included with different parameters
- Variable substitution now correctly prioritizes include parameters over template defines

## [2.1.0] - 2025-10-03

### Added
- **Include Resolution System**: Complete recursive include processing with wiki integration
  - Maximum depth limiting (16 levels) to prevent infinite recursion
  - Circular reference detection and prevention with visited tracking
  - Support for nested includes with proper AST substitution

- **Metadata Collection Pipeline**: Comprehensive document metadata extraction
  - Media file collection from all nested includes
  - Category collection (depth 0 only)
  - Redirect target detection (depth 0 only)
  - Forward-only variable substitution with parent parameter precedence

- **Build System**: Feature-gated preprocessor for WASM compatibility
  - Added `transform` feature flag for preprocessor and wiki client dependencies
  - WASM builds no longer require server-side dependencies (async-recursion, blake3, reqwest)
  - Server feature now includes transform feature automatically
  - Cleaner separation between client-side parsing and server-side processing

## [2.0.22] - 2025-09-29

### Added
- **Docker Support**: Complete containerization for development and deployment
  - Added optimized multi-stage Dockerfile with dependency caching
  - Docker Compose configuration for local development
  - GitHub Actions workflow for automated Docker image builds and publishing
  - Container runs on port 9000 with PostgreSQL integration support
  - Production-ready container with security best practices (non-root user)

## [2.0.21] - 2025-09-29

### Added
- **Web Server Infrastructure**: Complete HTTP API server implementation
  - Added `sevenmark-server` binary with Axum-based web framework
  - Database connectivity with SeaORM and PostgreSQL support
  - Comprehensive error handling system with structured HTTP responses
  - Environment-based configuration management with `.env` file support
  - Structured logging with tracing and log rotation capabilities

- **REST API Framework**: Full API infrastructure for SevenMark processing
  - Health check endpoints (`/v0/health_check`) with OpenAPI documentation
  - Modular API routing system with versioned endpoints (`v0/`)
  - Auto-generated OpenAPI/Swagger documentation with utoipa

- **Database Integration**: Production-ready database layer
  - Connection pooling with configurable min/max connections
  - Automatic connection retry and error recovery
  - Environment-specific database configuration
  - Integrated transaction support for complex operations

- **Development Tools**: Enhanced development and deployment support
  - Conditional compilation with `server` feature flag
  - WASM compatibility maintained alongside server features
  - Structured application state management
  - Comprehensive error protocol definitions

### Fixed
- **Build System**: Resolved filename collision between binary and library targets
  - Renamed main binary from `sevenmark` to `sevenmark-server` to prevent conflicts with library name
  - Fixed compilation errors in error handling system by adding missing `NotFound(_)` enum variant coverage
  - Resolved import path issue in health check routes (`super::health_check` → `super::health_check::health_check`)

## [2.0.20] - 2025-09-29

### Added
- **Variable Substitution System**: Complete implementation of template variable functionality
  - Forward-only variable resolution prevents circular dependencies
  - `{{{#define #var1="value" #var2="[var(var1)]"}}}` supports nested variable references
  - `[var(name)]` substitution works in MediaElement URLs and other contexts
  - Real-time variable expansion during preprocessing with proper composition
  - Enhanced preprocessor with 2-stage architecture: variable substitution → information collection

### Enhanced
- **Preprocessing Pipeline**: Restructured for better variable handling
  - Stage 1: `substitute_all_variables_in_ast()` resolves all variable references
  - Stage 2: `traverse_elements_and_collect_preprocess_info()` collects metadata
  - Variables are resolved before media URL collection for accurate preprocessing info
  - Improved performance with single-pass variable resolution

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