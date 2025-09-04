# Changelog

All notable changes to SevenMark parser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.11] - 2025-01-09

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

## [2.0.10] - 2025-01-09

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