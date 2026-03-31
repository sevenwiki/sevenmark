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
    fn render_keeps_allowed_layout_styles_and_drops_overlay_primitives() {
        let input = r#"{{{ #style="display:inline-block; position:fixed" #color="red" #dark-style="display:grid; z-index:1" #dark-color="blue" Styled text }}}"#;

        let ast = parse_document(input);
        let html = render_document(&ast, &RenderConfig::default());

        assert!(
            html.contains(" style=\""),
            "expected inline style attribute, got:\n{html}"
        );
        assert!(
            html.contains("display: inline-block"),
            "expected layout display to survive sanitization, got:\n{html}"
        );
        assert!(
            !html.contains("position: fixed"),
            "expected overlay positioning to be removed, got:\n{html}"
        );
        assert!(
            html.contains("data-dark-style=\"display: grid; color: #00f\""),
            "expected allowed dark style fragments to survive sanitization, got:\n{html}"
        );
        assert!(
            !html.contains("z-index"),
            "expected z-index to be removed, got:\n{html}"
        );
    }

    #[test]
    fn render_omits_style_attributes_when_only_blocked_properties_remain() {
        let input = r#"{{{ #style="position:fixed; z-index:1" #dark-style="inset:0; pointer-events:none" Unsafe only }}}"#;

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
