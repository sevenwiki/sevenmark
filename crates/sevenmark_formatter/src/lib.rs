mod config;
mod format;

pub use config::FormatConfig;

use pretty::Arena;
use sevenmark_ast::Element;

use format::element::format_elements;

/// Format a SevenMark AST back into source text.
pub fn format_document(elements: &[Element], config: &FormatConfig) -> String {
    let arena = Arena::new();
    let doc = format_elements(&arena, elements, config);
    let mut output = String::new();
    doc.render_fmt(config.width, &mut output).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    use sevenmark_html::{RenderConfig, render_document};
    use sevenmark_parser::core::parse_document;

    fn roundtrip(input: &str) -> String {
        let ast = parse_document(input);
        format_document(&ast, &FormatConfig::default())
    }

    fn normalize_newlines(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    fn assert_ast_roundtrip_stable(input: &str, context: &str) {
        let normalized = normalize_newlines(input);
        let ast1 = parse_document(&normalized);
        let formatted1 = format_document(&ast1, &FormatConfig::default());
        let ast2 = parse_document(&formatted1);
        let formatted2 = format_document(&ast2, &FormatConfig::default());
        assert_eq!(
            formatted1, formatted2,
            "Format not idempotent for {context}"
        );
    }

    fn assert_render_equivalent(input: &str, context: &str) {
        let normalized = normalize_newlines(input);
        let ast_before = parse_document(&normalized);
        let html_before = render_document(&ast_before, &RenderConfig::default());

        let formatted = format_document(&ast_before, &FormatConfig::default());
        let ast_after = parse_document(&formatted);
        let html_after = render_document(&ast_after, &RenderConfig::default());

        assert_eq!(
            html_before, html_after,
            "Rendered HTML mismatch for {context}\nformatted:\n{formatted}"
        );
    }

    #[test]
    fn test_plain_text() {
        assert_eq!(roundtrip("hello world"), "hello world");
    }

    #[test]
    fn test_bold() {
        assert_eq!(roundtrip("**bold**"), "**bold**");
    }

    #[test]
    fn test_italic() {
        assert_eq!(roundtrip("*italic*"), "*italic*");
    }

    #[test]
    fn test_strikethrough() {
        assert_eq!(roundtrip("~~struck~~"), "~~struck~~");
    }

    #[test]
    fn test_underline() {
        assert_eq!(roundtrip("__under__"), "__under__");
    }

    #[test]
    fn test_header() {
        assert_eq!(roundtrip("# Title"), "# Title\n");
    }

    #[test]
    fn test_folded_header() {
        assert_eq!(roundtrip("#! Folded"), "#! Folded\n");
    }

    #[test]
    fn test_hard_break() {
        assert_eq!(roundtrip("[br]"), "[br]");
    }

    #[test]
    fn test_hline() {
        assert_eq!(roundtrip("----"), "----\n");
    }

    #[test]
    fn test_escape() {
        assert_eq!(roundtrip("\\*"), "\\*");
    }

    #[test]
    fn test_variable() {
        assert_eq!(roundtrip("[var(x)]"), "[var(x)]");
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            roundtrip("{{{#code #lang=\"rust\" fn main() {} }}}"),
            "{{{#code #lang=\"rust\"\nfn main() {} }}}"
        );
    }

    #[test]
    fn test_multiline_text() {
        let input = "line1\nline2\nline3";
        let output = roundtrip(input);
        assert_eq!(output, "line1\nline2\nline3");
    }

    #[test]
    fn test_inline_comment_regression() {
        let input = "Text before comment // This is an inline comment\nText after comment.";
        assert_ast_roundtrip_stable(input, "inline comment newline separator");
    }

    #[test]
    fn test_fixture_ast_roundtrip_stability() {
        let categories = [
            "brace",
            "comment",
            "complex",
            "escape",
            "fold",
            "if",
            "macro",
            "markdown",
            "codemirror",
        ];
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tc");
        let mut checked = 0usize;

        for category in categories {
            let input_dir = fixtures_root.join(category).join("input");
            let Ok(entries) = fs::read_dir(&input_dir) else {
                continue;
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("sm") {
                    continue;
                }

                checked += 1;
                let input = fs::read_to_string(&path).unwrap();
                assert_ast_roundtrip_stable(&input, &path.display().to_string());
            }
        }

        assert!(checked > 0, "No fixture files were checked");
    }

    #[test]
    fn test_fixture_render_equivalence() {
        let categories = [
            "brace",
            "comment",
            "complex",
            "escape",
            "fold",
            "if",
            "macro",
            "markdown",
            "codemirror",
        ];
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tc");
        let mut checked = 0usize;

        for category in categories {
            let input_dir = fixtures_root.join(category).join("input");
            let Ok(entries) = fs::read_dir(&input_dir) else {
                continue;
            };

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("sm") {
                    continue;
                }

                checked += 1;
                let input = fs::read_to_string(&path).unwrap();
                assert_render_equivalent(&input, &path.display().to_string());
            }
        }

        assert!(checked > 0, "No fixture files were checked");
    }
}
