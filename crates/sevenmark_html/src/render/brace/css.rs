use maud::{Markup, PreEscaped, html};
use sevenmark_ast::{Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

fn is_style_close_boundary(b: u8) -> bool {
    b.is_ascii_whitespace() || matches!(b, b'>' | b'/')
}

fn sanitize_style_close_tag(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = String::with_capacity(value.len());
    let mut i = 0usize;

    while let Some(rel) = value[i..].find("</") {
        let start = i + rel;
        out.push_str(&value[i..start]);

        let tag_start = start + 2;
        let tag_end = tag_start + 5; // "style"
        if tag_end <= bytes.len() && bytes[tag_start..tag_end].eq_ignore_ascii_case(b"style") {
            // Accept only valid closing-tag boundaries after `style`, e.g.
            // `</style>`, `</style   >`, or `</style foo=bar>`.
            let boundary_ok = tag_end == bytes.len() || is_style_close_boundary(bytes[tag_end]);
            if boundary_ok {
                out.push_str("<\\/");
                out.push_str(&value[tag_start..tag_end]);
                i = tag_end;
                continue;
            }
        }

        out.push_str("</");
        i = tag_start;
    }

    out.push_str(&value[i..]);
    out
}

pub fn render(span: &Span, parameters: &Parameters, value: &str, ctx: &RenderContext) -> Markup {
    let merged_class = utils::merge_class(classes::CSS, parameters);
    let sanitized_css = super::super::sanitize::sanitize_css_block(value);
    let safe_css = sanitize_style_close_tag(&sanitized_css);
    let dark_style = utils::build_dark_style(parameters);

    html! {
        style
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            data-dark-style=[dark_style]
        { (PreEscaped(safe_css)) }
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_style_close_tag;

    #[test]
    fn sanitizes_case_insensitive_style_close_tag() {
        assert_eq!(
            sanitize_style_close_tag("a</sTyle>b"),
            "a<\\/sTyle>b".to_string()
        );
    }

    #[test]
    fn sanitizes_style_close_tag_with_whitespace() {
        assert_eq!(
            sanitize_style_close_tag("a</STYLE   >b"),
            "a<\\/STYLE   >b".to_string()
        );
    }

    #[test]
    fn sanitizes_style_close_tag_with_attributes() {
        assert_eq!(
            sanitize_style_close_tag("a</style foo=bar>b"),
            "a<\\/style foo=bar>b".to_string()
        );
    }

    #[test]
    fn does_not_sanitize_style_prefix_of_longer_tag_name() {
        assert_eq!(
            sanitize_style_close_tag("a</stylex>b"),
            "a</stylex>b".to_string()
        );
    }

    #[test]
    fn does_not_sanitize_hyphenated_tag_name() {
        assert_eq!(
            sanitize_style_close_tag("a</style-foo>b"),
            "a</style-foo>b".to_string()
        );
    }
}
