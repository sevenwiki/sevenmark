//! Styled element rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(
    span: &Span,
    parameters: &Parameters,
    children: &[Element],
    ctx: &mut RenderContext,
) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let content = render_elements(children, ctx);
    ctx.exit_suppress_soft_breaks();

    let style = utils::build_style(parameters);
    // Keep base renderer class and append optional user-defined #class.
    let merged_class = utils::merge_class(classes::STYLED, parameters);
    let dark_style = utils::build_dark_style(parameters);

    html! {
        span
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
            data-dark-style=[dark_style]
        { (content) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RenderConfig, render::render_document};
    use sevenmark_parser::core::parse_document;

    #[test]
    fn render_keeps_safe_style_fragments_and_drops_unsafe_ones() {
        let input = r#"{{{ #style="position:fixed" #color="red" #dark-style="position:fixed" #dark-color="blue" Styled text }}}"#;

        let ast = parse_document(input);
        let html = render_document(&ast, &RenderConfig::default());

        assert!(
            html.contains(" style=\""),
            "expected sanitized inline style attribute, got:\n{html}"
        );
        assert!(
            html.contains("style=\"color:"),
            "expected safe inline color to survive sanitization, got:\n{html}"
        );
        assert!(
            html.contains("data-dark-style=\"color:"),
            "expected safe dark color to survive sanitization, got:\n{html}"
        );
        assert!(
            !html.contains("position"),
            "expected unsafe style fragments to be removed, got:\n{html}"
        );
    }

    #[test]
    fn render_omits_style_attributes_when_sanitizer_removes_everything() {
        let input = r#"{{{ #style="position:fixed" #dark-style="position:fixed" Unsafe only }}}"#;

        let ast = parse_document(input);
        let html = render_document(&ast, &RenderConfig::default());

        assert!(
            !html.contains(" style=\""),
            "expected empty inline style attribute to be omitted, got:\n{html}"
        );
        assert!(
            !html.contains("data-dark-style=\""),
            "expected empty dark style attribute to be omitted, got:\n{html}"
        );
    }
}
