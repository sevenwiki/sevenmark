# SevenMark Development Commands

## Build Commands
```bash
# Standard build
cargo build

# Release build (optimized)
cargo build --release

# Check code without building
cargo check

# Build with server features (default)
cargo build --features server

# Build without default features
cargo build --no-default-features
```

## WASM Build Commands
```bash
# Add WASM target (first time only)
rustup target add wasm32-unknown-unknown

# Build for web browsers
wasm-pack build --target web --features wasm --no-default-features

# Build for bundlers (webpack/vite)
wasm-pack build --target bundler --features wasm --no-default-features

# Build for Node.js/VS Code extensions
wasm-pack build --target nodejs --features wasm --no-default-features
```

## Run Commands
```bash
# Run main server
cargo run

# Run parser without preprocessing (reads ToParse.txt → ParseResult.json)
cargo run --bin parse

# Run parser with preprocessing (reads ToParse.txt → ParseResult.json + PreprocessInfo.json)
cargo run --bin svm_file

# Run Monaco editor integration
cargo run --bin monaco
```

## Testing Commands
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific module
cargo test comprehensive_parser_tests
```

## Code Quality Commands
```bash
# Run linter
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Windows-Specific System Commands
```powershell
# List files/directories
dir
ls  # Works in PowerShell

# Change directory
cd path\to\directory

# Find files
Get-ChildItem -Recurse -Filter "*.rs"

# Search in files (grep equivalent)
Select-String -Pattern "pattern" -Path "*.rs"

# Git commands (same as Unix)
git status
git add .
git commit -m "message"
git push
```

## Development Workflow Files
- **ToParse.txt**: Input test file for parser development
- **ParseResult.json**: Serialized AST output
- **PreprocessInfo.json**: Metadata from preprocessing (includes, categories, media, redirects)
- **visualization.html**: Parse tree visualization tool