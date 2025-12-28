# Changelog

All notable changes to SevenMark parser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.8.13] - 2025-12-29

### Added
- **sevenmark-html**: Added `file_base_url` to `RenderConfig` for CDN URL prefixes
  - File/media URLs now support base URL configuration (e.g., Cloudflare CDN)
  - Image `src` attributes constructed as `format!("{}{}", file_base_url, file_url)`
  - Updated all examples and server request to include `file_base_url` field

### Changed
- **sevenmark-server**: `RenderDiscussionRequest` now accepts base URL configurations
  - Added `file_base_url`, `document_base_url`, `category_base_url` fields (consistent with `render-document`)
  - Discussion rendering no longer uses `DISCUSSION_CONFIG` const (creates config from request)

## [2.8.12] - 2025-12-28

### Changed
- **MediaElement**: Restructured `ResolvedMediaInfo` to support multiple parameters
  - New `ResolvedFile` struct: `url`, `is_valid` (for `#file` parameter)
  - New `ResolvedDoc` struct: `title`, `is_valid` (for `#document`, `#category` parameters)
  - `ResolvedMediaInfo` now has separate fields: `file`, `document`, `category`, `url`
  - Enables combinations like `[[#file="a.png" #url="https://..."]]` (image + external link)
  - URL generation moved from transform to HTML renderer for consistency with `edit_url`

- **sevenmark-html**: Added base URL configuration for internal links
  - `RenderConfig` now includes `document_base_url` and `category_base_url` fields
  - HTML renderer generates URLs using config (e.g., `format!("{}{}", base, title)`)
  - New CSS classes: `sm-link-invalid`, `sm-image-broken` for invalid links/images
  - Media rendering now supports image+link combinations

- **sevenmark-server**: Updated API request structure
  - `RenderDocumentRequest` now requires `document_base_url` and `category_base_url` fields
  - Client provides complete URL prefixes for document/category links
  - New render example added to sevenmark-server (with DB connection)

### Fixed
- **sevenmark-transform**: Fixed `#url` parameter not being resolved
  - `media.is_empty()` early return removed; now always traverses AST for `#url` processing
  - External links (`#url`) no longer require DB queries

## [2.8.11] - 2025-12-28

### Fixed
- **sevenmark-parser**: Fixed multiple mentions not being parsed
  - `text_parser` was consuming `<` as regular text, preventing subsequent mentions from being dispatched
  - Added `<` to excluded characters in `text_parser`

## [2.8.10] - 2025-12-28

### Added
- **sevenmark-server**: New `/v0/render-discussion` endpoint for discussion messages
  - `RenderDiscussionRequest`: `content` only (no `edit_url`)
  - `RenderedDiscussion`: `html`, `user_mentions` only (no `references`, `sections`, `categories`)
  - Uses `DISCUSSION_CONFIG` for rendering without edit links

### Changed
- **sevenmark-html**: `render_document` now takes `RenderConfig<'a>` instead of `&str`
  - `RenderConfig { edit_url: Option<&'a str> }` - controls edit link rendering
  - `DISCUSSION_CONFIG: RenderConfig<'static>` - const for discussion rendering (no edit links)
  - Edit links only rendered when `edit_url` is `Some(...)`

- **sevenmark-server**: Renamed render endpoint
  - `/v0/render` → `/v0/render-document`
  - Function renamed: `render_endpoint` → `render_document`

## [2.8.9] - 2025-12-28

### Added
- **Mention Elements**: New mention syntax for user and discussion references
  - `<@uuid>` - User mention with standard UUID format
  - `<#uuid>` - Discussion/document mention with standard UUID format
  - `MentionType` enum: `User`, `Discussion`
  - `MentionElement` struct with `location`, `mention_type`, `uuid` fields
  - HTML rendering: `<span class="sm-mention-user" data-uuid="..."></span>`
  - Frontend can hydrate mentions via `data-uuid` attribute

- **Parser Infrastructure**: New `<` character dispatch in element parser
  - `mention/` parser module with `mention_user.rs`, `mention_discussion.rs`
  - `token_angle_bracket.rs` fallback for non-mention `<` characters
  - UUID validation: 8-4-4-4-12 hex format (36 characters)

- **sevenmark-transform**: User mention collection in preprocessor
  - `PreProcessedDocument.user_mentions: HashSet<String>` - collected user UUIDs
  - Only user mentions (`<@uuid>`) are collected, discussion mentions are not

## [2.8.8] - 2025-12-23

### Changed
- **sevenmark-server**: `RenderDocumentRequest` now accepts `edit_url` directly instead of `document_title`
  - Client provides complete edit URL (e.g., `/edit/Document/대문`)
  - Server no longer constructs edit URL internally
  - Removed `FRONTEND_URL` environment variable requirement

### Removed
- **sevenmark-server**: Removed `frontend_url` from `ServerConfig`
  - No longer needed as client provides complete edit URL

## [2.8.6] - 2025-12-23

### Fixed
- **sevenmark-transform**: Fixed column calculation for multi-byte Unicode characters
  - Changed from byte offset difference to UTF-16 code units count
  - Monaco Editor uses UTF-16 code units for column positions ([microsoft/monaco-editor#3134](https://github.com/microsoft/monaco-editor/issues/3134))
  - Fixes incorrect column numbers for CJK characters (한글, 日本語, 中文) and emoji

## [2.8.2] - 2025-12-20

### Changed
- **sevenmark-html Optimization**: Reduced unnecessary allocations
  - `RenderContext::add_footnote()` returns `&str` instead of `String` (avoids clone)
  - `render_document()` defers footnote list rendering until needed

### Fixed
- **sevenmark-transform**: Collapsed nested if statement per clippy suggestion

## [2.8.1] - 2025-12-20

### Changed
- **Server API Endpoint**: Renamed `/v0/parse` to `/v0/render`
  - Now returns rendered HTML instead of raw AST
  - `RenderDocumentRequest`: `content` (SevenMark source) + `edit_url` (edit link prefix)
  - `RenderedDocument`: `html`, `categories`, `redirect`, `references`, `sections`
  - `sections` field contains `SectionInfo` with byte offsets for section editing

### Added
- **sevenmark-html Integration**: Server now uses sevenmark-html for rendering
  - Added `sevenmark-html` dependency to `sevenmark-server`
  - Full pipeline: parse → preprocess → postprocess → render to HTML

## [2.8.0] - 2025-12-20

### Added
- **New `sevenmark-html` Crate**: Server-side semantic HTML renderer using maud
  - Transforms SevenMark AST to nested HTML structure for SEO and SSR
  - Section tree building with O(N) algorithm for hierarchical document structure
  - Nested `<section>` elements enable cascading fold behavior (child sections hidden when parent folds)
  - Section path calculation (1, 1.1, 1.1.1, 1.2, 2...) for hierarchical navigation
  - Edit links with section index for section-based editing (`/edit/doc?section=3`)
  - CSS class constants (`sm-*` prefix) for frontend styling
  - Footnote collection and rendering with hover tooltips

- **Section Tree Structure**: Hierarchical document organization
  - `Section` struct: header, section_path, content, children
  - `SectionTree` struct: preamble (content before first header), sections
  - `build_section_tree()` converts flat AST to nested tree in single pass
  - Headers with higher level numbers become children (H2 is child of H1)

- **Header Rendering**: Enhanced header output structure
  - `<span class="sm-section-path">1.1.</span>` - Section number
  - `<span class="sm-header-content">Title</span>` - Header text
  - `<a class="sm-edit-link" href="/edit/doc?section=N">[Edit]</a>` - Edit link
  - `id="s-1.1"` uses section_path, `data-section="N"` uses section_index

- **Module Structure**: Organized render modules
  - `render/document.rs` - Document-level rendering with section tree
  - `render/element.rs` - Element dispatch and rendering
  - `render/markdown/` - Text formatting (bold, italic, header, etc.)
  - `render/brace/` - Brace elements (code, table, list, footnote, etc.)
  - `render/bracket/` - Media elements
  - `render/macro/` - Macros (newline, hline, timenow, age, footnote list)
  - `section.rs` - Section tree data structures and builder

## [2.7.19] - 2025-12-20

### Changed
- **`CodeElement.content` Type**: `Vec<SevenMarkElement>` → `String`
  - Code block content is now stored as raw string, similar to `TeXElement`
  - Simplified parsing using `take_until("}}}")` instead of complex token-based parsing
  - Removed escape sequence processing inside code blocks
  - Deleted `parser/brace/code/` directory (code_content.rs, code_text.rs)

### Fixed
- **Test Expected File Generation**: Fixed path in `gen_expected.rs` and `gen_monaco_expected.rs`
  - Changed `../tc/` to `tc/` for correct path resolution from project root

## [2.7.18] - 2025-12-17

### Added
- **Section Range Information**: `PreProcessedDocument` and `ProcessedDocument` now include `sections` field
  - `SectionInfo` struct with `section_index`, `level`, `start`, `end` (byte offsets)
  - Calculated during `collect_metadata` traversal (O(n) stack-based algorithm)
  - Frontend can directly use section ranges without AST traversal
  - Useful for section editing, TOC generation, scroll sync

- **`SevenMarkElement::location()` Method**: Added location accessor to enum
  - Returns `Option<&Location>` for elements with location data
  - `None` for location-less elements (Null, FootNote, TimeNow, NewLine, HLine)

- **`DocumentReference` Struct**: New struct for document references
  - Replaces `(DocumentNamespace, String)` tuple in `references` field
  - Consistent with `MediaReference`, `RedirectReference` pattern
  - Fields: `namespace: DocumentNamespace`, `title: String`

### Changed
- **`references` Field Type**: `HashSet<(DocumentNamespace, String)>` → `HashSet<DocumentReference>`
  - Improved type consistency across document/media/redirect references

## [2.7.16] - 2025-12-16

### Added
- **Footnote Parameters Support**: `FootnoteElement` now supports parameters
  - New syntax: `{{{#fn #display="*" #id="note1" content}}}`
  - `FootnoteElement` now has `parameters: Parameters` field
  - Enables custom display text, IDs, and other metadata for footnotes
  - Frontend can control footnote rendering based on parameters

### Removed
- **sevenmark-renderer Crate**: Removed from workspace
  - Renderer functionality moved to separate repository/branch
  - Focus on parser and transform layers for AST generation
  - Frontend handles HTML rendering directly from AST

## [2.7.15] - 2025-12-15

### Added
- **New `sevenmark-renderer` Crate**: Server-side HTML rendering engine
  - Maud-based HTML generation from AST
  - Full element support: headers, lists, tables, code blocks, TeX, fold, ruby, styled elements, etc.
  - CSS utilities with parameter-to-style mapping (`#color`, `#bgcolor`, `#size`, `#width`, etc.)
  - Dark mode support via CSS variables (`--sm-text`, `--sm-bg`, etc.)
  - KaTeX integration for math rendering
  - Prism.js integration for syntax highlighting

- **Explicit Display Mode Control**: `#block` and `#inline` parameters for styled elements
  - `#block` → adds `display: block` to CSS
  - `#inline` → adds `display: inline` to CSS
  - Allows user override via more specific `#style` values
  - Simplified renderer: always uses `<span>`, display controlled by CSS

### Changed
- **Styled Element Rendering**: Removed auto-detection of block vs inline
  - Previously auto-detected based on CSS properties (padding, margin, float, etc.)
  - Now requires explicit `#block` parameter for block-level layout
  - More predictable and user-controlled behavior

## [2.7.13] - 2025-12-11

### Added
- **Redirect Namespace Support**: Added namespace parameter support for redirect elements
  - `RedirectElement` now has `parameters: Parameters` field (same as `IncludeElement`)
  - New syntax: `{{{#redirect #namespace="File" image.png}}}` for file redirects
  - New syntax: `{{{#redirect #namespace="Category" category_name}}}` for category redirects
  - Default namespace is `Document` for backward compatibility
  - New `RedirectReference` struct with `namespace` and `title` fields

### Changed
- **Redirect Output Structure**: Changed from `Option<String>` to `Option<RedirectReference>`
  - `PreProcessedDocument.redirect` now contains namespace info
  - `ProcessedDocument.redirect` now contains namespace info
  - **Breaking Change**: Frontend needs to access `redirect.namespace` and `redirect.title` instead of plain string

## [2.7.12] - 2025-12-11

### Changed
- **References Collection**: Switched to render-based reference collection
  - Removed Include case from `collect_references_recursive()`
  - Only directly included documents (1-depth) are added to references
  - 2+ depth includes (B→C when A includes B) are excluded since they're not rendered
  - Categories and MediaElements from included content are still collected

## [2.7.11] - 2025-12-11

### Fixed
- **References Collection Bug**: Fixed missing direct include references after substitution
  - After `substitute_includes()`, Include nodes' `content` is overwritten with included AST
  - `collect_references()` couldn't extract original document titles from substituted Include nodes
  - Now merges `includes_to_fetch` (collected before substitution) into `all_references`
  - 1-depth includes (A→B): from `includes_to_fetch`
  - 2+ depth includes (B→C): from `collect_references()` (unsubstituted Include nodes)

## [2.7.10] - 2025-12-11

### Changed
- **References Field**: Renamed `includes` to `references` in `PreProcessedDocument` and `ProcessedDocument`
  - `references` now contains all document references (includes, categories, media) for backlink tracking
  - Collected after `substitute_includes()` to capture references from included documents (MediaWiki-style)
  - Types affected: `PreProcessedDocument.references`, `ProcessedDocument.references`

### Added
- **Comprehensive Reference Collection**: New `collect_references()` function in preprocessor
  - Collects `{{{#include}}}` elements → `(namespace, title)`
  - Collects `{{{#category}}}` elements → `(Category, name)`
  - Collects `MediaElement` parameters: `#file` → `(File, title)`, `#document` → `(Document, title)`, `#category` → `(Category, title)`
  - Enables complete backlink tracking for all referenced documents

### Removed
- **Legacy Code Cleanup**: Removed unused `sevenmark-server/src/sevenmark/` directory
  - Was a duplicate of `sevenmark-transform` code with `#[cfg(feature = "transform")]` guards
  - Server already uses `sevenmark-transform` as dependency

## [2.7.7] - 2025-12-10

### Changed
- **Workspace Structure**: Reorganized crates into `crates/` folder
  - Moved `sevenmark-parser`, `sevenmark-transform`, `sevenmark-server` into `crates/` directory
  - Changed `members` from explicit list to glob pattern: `["crates/*"]`
  - Follows sevenwiki-server project structure convention

- **Dependency Centralization**: Consolidated all dependencies in workspace root
  - All shared dependencies now defined in root `Cargo.toml` under `[workspace.dependencies]`
  - Individual crates use `{ workspace = true }` references instead of version numbers
  - Centralized: `winnow`, `line-span`, `paste`, `axum`, `tokio`, `utoipa`, `utoipa-swagger-ui`, `uuid`, `dotenvy`, `tracing-subscriber`, `tracing-appender`, `wasm-bindgen`, `js-sys`, `web-sys`
  - Easier version management and consistency across all crates

## [2.7.6] - 2025-12-09

### Performance
- **Parser Clone Optimization**: Eliminated expensive `HashSet` cloning during parsing
  - Changed `line_starts: HashSet<usize>` to `Rc<HashSet<usize>>` in `ParseContext`
  - winnow checkpoint operations now clone reference counter (O(1)) instead of full HashSet (O(n))
  - Significantly reduces memory allocation overhead for documents with many lines
  - Throughput now remains constant regardless of document size (~31,000 KB/s)

- **Removed Unnecessary Clone in `with_depth`**: Simplified recursion depth helper
  - Removed redundant `input.clone()` that served no purpose
  - Direct mutation of `ParseContext` instead of clone-modify-replace pattern
  - Reduces overhead for every nested element parse (bold, italic, tables, lists, etc.)

## [2.7.4] - 2025-12-08

### Changed
- **Link Existence Check Optimization**: Separated content fetching from existence checking
  - New `check_documents_exist()` function for lightweight link validation (red/blue link coloring)
  - No longer fetches full document content just to check if a link exists
  - Memory usage reduced from ~50KB/link to ~0.1KB/link
  - Query optimization: grouped namespace conditions with `IN` clause instead of individual `OR` conditions
  - `fetch_documents_batch()` also optimized with same `IN` clause pattern

### Added
- **DocumentExistence Type**: New lightweight response type for link existence checks
  - Contains only `namespace`, `title`, `exists`, and `file_url` fields
  - Used by postprocessor for efficient `is_valid` resolution

- **External Link Distinction**: `is_valid` field changed from `bool` to `Option<bool>`
  - `Some(true)` = internal document exists (blue link)
  - `Some(false)` = internal document missing (red link)
  - `None` = external URL (no existence concept, separate styling)

## [2.7.3] - 2025-12-07

### Changed
- **Parser Performance**: Refactored `element_parser` from `alt` to `dispatch` pattern
  - Uses winnow's `dispatch!` macro with `peek(any)` for O(1) first-character branching
  - Replaces sequential O(n) parser attempts with direct character-based dispatch
  - Significant performance improvement for plain text parsing (most common case)
  - Grouped parsers by first character: `\`, `/`, `{`, `}`, `[`, `]`, `#`, `-`, `*`, `_`, `~`, `^`, `,`, `\n`
  - Fallback `_ => text_parser` for all other characters

## [2.7.2] - 2025-12-06

### Changed
- **API Route Naming**: Changed health check endpoint from snake_case to kebab-case
  - `/health_check` → `/health-check`
  - Aligns with REST API URL naming conventions

## [2.7.0] - 2025-12-04

### Added
- **Section Editing Support**: Added `section_index` to Header elements for frontend section editing
  - `Header` struct now includes `section_index: usize` field
  - Section indices are assigned sequentially (1, 2, 3...) during parsing
  - `ParseContext` tracks section counter with `next_section_index()` method
  - Enables MediaWiki-style section editing: frontend can use `section_index` + `location` to extract section text

- **WASM Byte Offset Export**: New `parse_sevenmark()` function for section editing use case
  - Returns AST with byte offset locations (`start`, `end`)
  - Separate from `parse_sevenmark_to_monaco()` which returns line/column format
  - Frontend uses byte offsets for `text.slice(start, end)` operations

### Changed
- **JSON Output**: Switched to compact JSON for all WASM exports
  - `convert_ast_to_line_column_json()` now uses `serde_json::to_string()` instead of `to_string_pretty()`
  - Smaller payload size for better performance
  - No functional impact (frontend parses JSON anyway)

## [2.6.9] - 2025-11-29

### Fixed
- **Expression Parser Recursion Protection**: Added depth limiting to prevent stack overflow
  - `group_parser`: Now applies `with_depth` when parsing `(...)` groups
  - `function_call_parser`: Now applies `with_depth` when parsing function arguments
  - Prevents DoS attacks via deeply nested expressions like `((((((x))))))` or `int(int(int(x)))`
  - Shares depth limit with element parser (default: 16 levels total)

## [2.6.8] - 2025-11-29

### Added
- **Logical Operator Location Tracking**: Added location information to logical operators (`||`, `&&`, `!`)
  - New `LogicalOperatorKind` enum: `Or`, `And`, `Not`
  - New `LogicalOperator` struct with `location: Location` and `kind: LogicalOperatorKind` fields
  - `Or`, `And`, `Not` Expression variants now include `operator: LogicalOperator` field
  - Enables precise syntax highlighting for all operators in expressions

### Changed
- **Expression Parser**: Logical operator parsers now capture operator positions
  - `or_operator_parser` captures `||` symbol location
  - `and_operator_parser` captures `&&` symbol location
  - `not_operator_location_parser` captures `!` symbol location
  - All operators (logical and comparison) now have dedicated location tracking

- **NOT Operator**: Changed to allow only single `!` operator
  - `!x` → valid
  - `!!x` → parse error (previously collapsed to just `x`)
  - `!(!x)` → valid (explicit double negation with parentheses)
  - Prevents silent consumption of redundant `!` operators
  - Clearer semantics and better error detection for typos

## [2.6.7] - 2025-11-29

### Added
- **Expression Location Tracking**: Added location information to all Expression AST nodes
  - All Expression variants now include `location: Location` field
  - `Or`, `And`, `Not`, `Comparison`, `FunctionCall`, `Group` variants converted to struct variants
  - `StringLiteral`, `NumberLiteral`, `BoolLiteral`, `Null` variants converted to struct variants with `value` field
  - `Element` variant remains tuple variant (inner SevenMarkElement already has location)
  - Location serialization controlled by `include_locations` feature flag

- **ComparisonOperator Location Tracking**: Restructured for location support
  - Renamed `ComparisonOperator` enum to `ComparisonOperatorKind`
  - New `ComparisonOperator` struct with `location: Location` and `kind: ComparisonOperatorKind` fields
  - Enables precise error reporting for comparison operators

### Changed
- **Expression Parser**: All parser functions now capture start/end positions
  - `or_parser`, `and_parser`, `not_parser`, `comparison_parser` track locations
  - `operand_parser` variants (`group_parser`, `function_call_parser`, literals) track locations
  - `comparison_operator_parser` captures operator position

- **Traversable**: Updated pattern matching for new struct variants
  - `traverse_expression` and `traverse_expression_ref` use `..` to ignore location fields

- **Expression Evaluator**: Updated for new Expression structure
  - Pattern matching uses struct variant syntax with `..`
  - `compare_values` accesses `operator.kind` instead of direct enum matching
  - Test helpers added for cleaner Expression construction

## [2.6.5] - 2025-11-28

### Changed
- **Parameter Parser**: Allow spaces around `=` in parameter syntax
  - `#key="value"`, `#key = "value"`, `#key= "value"`, `#key ="value"` all valid

## [2.6.4] - 2025-11-28

### Changed
- **AST Module Refactoring**: Split monolithic `ast.rs` into modular `ast/` folder structure
  - `mod.rs` - SevenMarkElement enum and re-exports
  - `location.rs` - Location, Parameter, Parameters types
  - `expression.rs` - Expression, ComparisonOperator, IfElement
  - `table.rs` - Table structures (TableElement, TableRowItem, TableCellItem)
  - `list.rs` - List structures (ListElement, ListContentItem)
  - `elements.rs` - Basic element structs (TextElement, StyledElement, etc.)
  - `traversable.rs` - Traversable trait implementation

### Added
- **Table/List Conditional Support**: Added `{{{#if}}}` support inside tables and lists
  - `TableRowItem::Conditional` - Conditional table rows with struct variant
  - `TableCellItem::Conditional` - Conditional table cells with struct variant
  - `ListContentItem::Conditional` - Conditional list items with struct variant
  - Syntax: `{{{#if condition :: [[item1]] [[item2]] }}}`
  - Preprocessor support for evaluating conditionals in table/list contexts

- **Traversable Enhancement**: Added `traverse_children_ref` for immutable AST traversal
  - Eliminates unnecessary cloning during metadata collection
  - Reduces O(N²)~O(N³) complexity to O(N) for collect operations
  - Used by preprocessor's `collect_metadata_recursive` and `collect_includes_recursive`

- **Test Infrastructure**: Added test case generation utilities
  - `gen_expected` example in sevenmark-parser for parser test expected files
  - `gen_monaco_expected` example in sevenmark-transform for Monaco position test files
  - New test cases for if expressions (basic_comparison, functions, logical_operators, etc.)
  - New test cases for table/list conditionals (table_row_conditional, table_cell_conditional, list_conditional)

## [2.6.1] - 2025-11-28

### Added
- **Boolean Literals**: Added `true` and `false` keyword support in expressions
  - Parser: `bool_literal_parser` handles `true`/`false` keywords
  - AST: New `Expression::BoolLiteral(bool)` variant
  - Evaluator: Properly handles boolean literal values
  - Enables patterns like `(5 > 3) == true` and `[var(enabled)] == false`

## [2.6.0] - 2025-11-28

### Added
- **Conditional Elements**: New `{{{#if condition :: content}}}` syntax for conditional rendering
  - Content is included in output only when condition evaluates to true
  - Processed during preprocessing phase alongside variable substitution
  - Supports nested elements and formatting inside conditional blocks

- **Expression Parser**: Complete expression evaluation system in `parser/expression/` module
  - Comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=`
  - Logical operators: `&&` (and), `||` (or), `!` (not)
  - Parentheses for grouping: `(a || b) && c`
  - Type conversion functions: `int()`, `len()`, `str()`
  - `null` keyword for null comparisons
  - Optional `::` delimiter to explicitly separate condition from content

- **Expression Evaluator**: Runtime condition evaluation in `sevenmark-transform`
  - Short-circuit evaluation: `false && X` and `true || X` skip right-side evaluation
  - Enables null-guard patterns: `[var(x)] != null && int([var(x)]) > 5`
  - Strict numeric comparison: `>`, `<`, `>=`, `<=` only work when both values are numeric
    - `"abc" < 5` → false (not silently converted to `0 < 5`)
    - `"10" > 5` → true (parseable strings work)
    - `null > 5` → false (null is not comparable)
  - Loose type coercion for equality only (e.g., `"5" == 5` is true)
  - Bool equality comparison supported (e.g., `(a > b) == (c > d)`)
  - Variable references via `[var(name)]` syntax
  - Null handling for undefined variables

- **Traversable Enhancement**: Added `for_each_content_vec` method to `Traversable` trait
  - Enables Vec-level operations on AST content collections
  - Used by preprocessor for conditional element expansion/removal
  - Complements existing `for_each_child` for element-level traversal

## [2.5.5] - 2025-11-13

### Changed
- **Error Handling Architecture**: Refactored to handler pattern for better modularity
  - Implemented domain-separated handler pattern inspired by sevenwiki-server architecture
  - Created `handlers/` directory with specialized error handlers per domain
    - `document_handler.rs`: Handles document-related errors (DocumentNotFound, DocumentRevisionNotFound)
    - `general_handler.rs`: Handles client errors (BadRequestError, ValidationError)
    - `system_handler.rs`: Handles system errors (SysInternalError, DatabaseError, NotFound)
  - Each handler provides two core functions:
    - `log_error(&Errors)`: Domain-specific logging with appropriate levels (warn/error/debug)
    - `map_response(&Errors) -> Option<(StatusCode, &str, Option<String>)>`: HTTP response mapping
  - Refactored `IntoResponse` implementation to use handler chain pattern with `or_else` composition
  - Improved extensibility: new error domains can be added by creating new handlers
  - Enhanced maintainability: each domain's error handling logic is now isolated and testable
  - Better separation of concerns: logging and response mapping separated by domain

## [2.5.1] - 2025-11-13

### Fixed
- **Server Build**: Fixed missing `server` feature activation for `sevenmark-transform`
  - Added explicit `features = ["server"]` to `sevenmark-server/Cargo.toml`
  - Resolves compilation errors: `ProcessedDocument` and `process_sevenmark` not found
  - Fixes Docker build failures due to missing preprocessor/postprocessor modules

## [2.5.0] - 2025-11-13

### Changed - BREAKING
- **Project Structure**: Migrated from monolithic crate to Cargo workspace architecture
  - Split into 3 specialized crates for better modularity and maintainability
  - **`sevenmark-parser`**: Pure parsing library with minimal dependencies (winnow, serde, line-span)
  - **`sevenmark-transform`**: AST preprocessing/postprocessing, position conversion, and WASM exports
  - **`sevenmark-server`**: REST API server with database integration
  - Each crate can now be used independently based on project needs

### Changed
- **Feature Flags**: Reorganized feature structure across workspace
  - `sevenmark-parser`: Simplified to only `include_locations` feature
    - Removed: `wasm`, `server`, `transform` features (no longer needed in parser)
  - `sevenmark-transform`: Added `server` (default) and `wasm` features
    - `server`: Enables preprocessing/postprocessing with database dependencies
    - `wasm`: Enables WebAssembly exports with location tracking
  - Server dependencies now optional in transform crate (sea-orm, uuid, utoipa, tracing)

- **WASM Build Location**: Moved from root to `sevenmark-transform/`
  - WASM function `parse_sevenmark_to_monaco()` now exported from transform crate
  - Build command: `cd sevenmark-transform && wasm-pack build --target bundler --features wasm --no-default-features`
  - `--no-default-features` required to exclude server dependencies incompatible with WASM
  - Location tracking automatically included in WASM builds

- **Crate Types**: Optimized library configuration
  - Parser: Standard `rlib` only (removed `cdylib`)
  - Transform: Both `cdylib` (for WASM) and `rlib` (for server)
  - Server: Binary crate only

### Removed
- **Parser Crate Cleanup**: Removed WASM-related code
  - Removed optional dependencies: `wasm-bindgen`, `js-sys`, `web-sys`
  - Removed `[lib] crate-type = ["cdylib", "rlib"]` configuration
  - Removed dead code: commented-out `parse_document_with_processing()` function

- **Server Crate Cleanup**: Removed unnecessary feature declarations
  - Removed empty `default = []` feature (no custom features needed)

### Infrastructure
- **CI/CD Workflows**: Updated for workspace architecture
  - Builds: `cargo build --release --workspace`
  - Tests: Separated per-package with appropriate feature flags
    - `cargo test -p sevenmark-parser --features include_locations`
    - `cargo test -p sevenmark-transform`
    - `cargo test -p sevenmark-server`
  - WASM tests: Execute from `sevenmark-transform/` directory with proper working directory

- **Docker**: Updated Dockerfile for multi-crate workspace
  - Copies all 3 crate Cargo.toml files for dependency caching
  - Builds with `cargo build --release -p sevenmark-server`
  - Updated Rust version to 1.91

- **GitHub Actions**: Fixed WASM release workflow
  - Release builds now run from correct directory with `working-directory: ./sevenmark-transform`
  - Properly packages WASM artifacts for web, bundler, and nodejs targets

### Documentation
- **README**: Complete rewrite for workspace structure
  - Added workspace architecture diagram showing crate relationships
  - Updated all build commands for multi-package project
  - Clarified WASM build requirements and exported function signature
  - Updated dependency lists organized by crate
  - Added migration guide for upgrading from 2.4.x

### Migration Guide

For users upgrading from 2.4.x to 2.5.0:

**Rust Library Usage:**
```rust
// BEFORE (2.4.x)
use sevenmark::parse_document;
use sevenmark::convert_ast_to_line_column_json;

// AFTER (2.5.0)
use sevenmark_parser::core::parse_document;
use sevenmark_transform::convert_ast_to_line_column_json;
```

**Cargo.toml Dependencies:**
```toml
# BEFORE (2.4.x)
[dependencies]
sevenmark = "2.4"

# AFTER (2.5.0) - Choose what you need:
[dependencies]
sevenmark-parser = "2.5"  # Just parsing
sevenmark-transform = "2.5"  # Parsing + preprocessing/postprocessing
# Note: sevenmark-server is a binary, not a library
```

**WASM Build Commands:**
```bash
# BEFORE (2.4.x)
wasm-pack build --target bundler --features wasm --no-default-features

# AFTER (2.5.0)
cd sevenmark-transform
wasm-pack build --target bundler --features wasm --no-default-features
```

**Server Deployment:**
```bash
# BEFORE (2.4.x)
cargo run --features server

# AFTER (2.5.0)
cargo run -p sevenmark-server
```

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