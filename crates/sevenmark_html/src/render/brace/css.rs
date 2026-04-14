use maud::{Markup, PreEscaped, html};
use sevenmark_ast::Span;

use crate::classes;
use crate::context::RenderContext;

use super::super::sanitize::escape_style_close_tag;

pub fn render(
    _span: &Span,
    value: &str,
    _ctx: &RenderContext,
) -> Markup {
    let sanitized_css = super::super::sanitize::sanitize_css_block(value);
    let safe_css = escape_style_close_tag(&sanitized_css);

    html! {
        style
            class=(classes::CSS)
        { (PreEscaped(safe_css)) }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::sanitize::escape_style_close_tag;
    use crate::test_support::{parse_fragment, render_html, selector};

    fn count_occurrences(haystack: &str, needle: &str) -> usize {
        haystack.match_indices(needle).count()
    }

    #[test]
    fn sanitizes_case_insensitive_style_close_tag() {
        assert_eq!(
            escape_style_close_tag("a</sTyle>b"),
            "a<\\/sTyle>b".to_string()
        );
    }

    #[test]
    fn sanitizes_style_close_tag_with_whitespace() {
        assert_eq!(
            escape_style_close_tag("a</STYLE   >b"),
            "a<\\/STYLE   >b".to_string()
        );
    }

    #[test]
    fn sanitizes_style_close_tag_with_attributes() {
        assert_eq!(
            escape_style_close_tag("a</style foo=bar>b"),
            "a<\\/style foo=bar>b".to_string()
        );
    }

    #[test]
    fn does_not_sanitize_style_prefix_of_longer_tag_name() {
        assert_eq!(
            escape_style_close_tag("a</stylex>b"),
            "a</stylex>b".to_string()
        );
    }

    #[test]
    fn does_not_sanitize_hyphenated_tag_name() {
        assert_eq!(
            escape_style_close_tag("a</style-foo>b"),
            "a</style-foo>b".to_string()
        );
    }

    #[test]
    fn render_sanitizes_css_block_and_escapes_style_close_sequences() {
        let input = r#"{{{#css
.card { font-family: "</style>"; color: red; background: url(evil.png); }
body { color: blue; }
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);
        let style = doc
            .select(&selector("style.sm-css"))
            .next()
            .expect("expected style element");

        assert_eq!(style.value().name(), "style", "expected style element in output, got:\n{html}");
        assert!(
            style.value().attr("data-start").is_none(),
            "expected non-visual style tags to omit span offsets, got:\n{html}"
        );
        assert!(
            style.value().attr("data-end").is_none(),
            "expected non-visual style tags to omit span offsets, got:\n{html}"
        );
        let css = style.inner_html();
        assert!(
            css.contains(".card"),
            "expected safe class selector to survive sanitization, got:\n{html}"
        );
        assert!(
            css.contains("color"),
            "expected safe property to survive sanitization, got:\n{html}"
        );
        assert!(
            html.contains("<\\/style>"),
            "expected embedded style-close sequence to be escaped, got:\n{html}"
        );
        assert!(
            !html.contains("url("),
            "expected dynamic URL value to be removed, got:\n{html}"
        );
        assert!(
            !css.contains("body"),
            "expected bare tag selector to be removed, got:\n{html}"
        );
        assert_eq!(
            count_occurrences(&html, "</style>"),
            1,
            "expected only the renderer's closing style tag to remain, got:\n{html}"
        );
    }

    #[test]
    fn css_blocks_do_not_emit_data_dk_or_shared_dark_style_rules() {
        let input = r#"{{{#css
.card { color: red; }
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);
        let styles = doc.select(&selector("style")).collect::<Vec<_>>();
        assert_eq!(styles.len(), 1, "expected only the authored css style tag, got:\n{html}");

        let style = styles[0].value();
        assert_eq!(style.attr("class"), Some("sm-css"));
        assert!(
            style.attr("data-dk").is_none(),
            "css blocks must not emit data-dk, got:\n{html}"
        );
        assert!(
            !html.contains(".dark [data-dk="),
            "css block dark params should be ignored instead of generating shared dark rules, got:\n{html}"
        );
    }
}
