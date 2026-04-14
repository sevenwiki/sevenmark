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
    let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(parameters));

    html! {
        (dark_tag)
        span
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
            data-dk=[dk]
        { (content) }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_support::{parse_fragment, render_html, selector};

    #[test]
    fn render_keeps_allowed_layout_styles_and_drops_overlay_primitives() {
        let input = r#"{{{ #style="display:inline-block; position:fixed" #color="red" #dark-style="display:grid; z-index:1" #dark-color="blue" Styled text }}}"#;
        let html = render_html(input);
        let doc = parse_fragment(&html);
        let styled = doc
            .select(&selector("span.sm-styled"))
            .next()
            .expect("expected styled span");

        assert!(
            styled.value().attr("style").is_some(),
            "expected inline style attribute, got:\n{html}"
        );
        let style = styled
            .value()
            .attr("style")
            .expect("styled span should have style attribute");
        assert_eq!(
            style, "display: inline-block; color: red",
            "expected layout display and color to survive sanitization, got:\n{html}"
        );
        assert!(
            !style.contains("position"),
            "expected overlay positioning to be removed, got:\n{html}"
        );
        assert!(
            html.contains("display: grid") && html.contains("color: #00f"),
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
        let html = render_html(input);
        let doc = parse_fragment(&html);
        let styled = doc
            .select(&selector("span.sm-styled"))
            .next()
            .expect("expected styled span");

        assert!(
            styled.value().attr("style").is_none(),
            "expected empty inline style attribute to be omitted, got:\n{html}"
        );
        assert!(
            !html.contains("data-dk"),
            "expected no dark style tag when all dark properties are blocked, got:\n{html}"
        );
    }
}
