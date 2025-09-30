# Task Completion Checklist

When completing a coding task in SevenMark, follow this checklist:

## 1. Code Quality
- [ ] Run `cargo fmt` to format code
- [ ] Run `cargo clippy` to check for linting issues
- [ ] Resolve all clippy warnings if reasonable

## 2. Testing
- [ ] Run `cargo test` to ensure all tests pass
- [ ] Add new tests if implementing new features
- [ ] Test with `cargo run --bin parse` for basic parsing
- [ ] Test with `cargo run --bin svm_file` for preprocessing features

## 3. Build Verification
- [ ] Run `cargo check` for quick compilation check
- [ ] Run `cargo build` to ensure successful build
- [ ] If changing public APIs, run `cargo build --release` to verify optimization

## 4. WASM Compatibility (if applicable)
- [ ] If changes affect parser core, test WASM build:
  ```bash
  wasm-pack build --target web --features wasm --no-default-features
  ```

## 5. Documentation
- [ ] Update doc comments for new/changed public APIs
- [ ] Update CLAUDE.md if architecture changed
- [ ] Update README.md if user-facing features changed

## 6. Performance
- [ ] If parser changes made, verify no performance regression:
  - Use `ToParse.txt` with sample content
  - Run `cargo run --bin parse` and check KB/s metric
  - Compare with baseline performance (>10 MB/s expected)

## 7. Git
- [ ] Review changes: `git diff`
- [ ] Stage changes: `git add .` or specific files
- [ ] Commit with descriptive message: `git commit -m "description"`
- [ ] Push if ready: `git push`

## Minimal Checklist (Quick Changes)
For small changes, at minimum:
1. `cargo fmt`
2. `cargo test`
3. `cargo check`