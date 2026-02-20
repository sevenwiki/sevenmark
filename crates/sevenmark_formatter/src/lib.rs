mod config;
mod format;

pub use config::FormatConfig;

use pretty::Arena;
use sevenmark_ast::Element;

use format::element::format_elements;

/// Format a SevenMark AST back into source text.
pub fn format_document(elements: &[Element], config: &FormatConfig) -> String {
    let arena = Arena::new();
    let doc = format_elements(&arena, elements);
    let mut output = String::new();
    doc.render_fmt(config.width, &mut output).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use sevenmark_parser::core::parse_document;

    fn roundtrip(input: &str) -> String {
        let ast = parse_document(input);
        format_document(&ast, &FormatConfig::default())
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
        assert_eq!(roundtrip("# Title"), "# Title");
    }

    #[test]
    fn test_folded_header() {
        assert_eq!(roundtrip("#! Folded"), "#! Folded");
    }

    #[test]
    fn test_hard_break() {
        assert_eq!(roundtrip("[br]"), "[br]");
    }

    #[test]
    fn test_hline() {
        assert_eq!(roundtrip("----"), "----");
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
            "{{{#code #lang=\"rust\" fn main() {} }}}"
        );
    }

    #[test]
    fn test_multiline_text() {
        let input = "line1\nline2\nline3";
        let output = roundtrip(input);
        assert_eq!(output, "line1\nline2\nline3");
    }
}
