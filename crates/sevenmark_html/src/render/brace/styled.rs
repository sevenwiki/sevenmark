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

    let lk = ctx.add_light_style(utils::build_style(parameters));
    // Keep base renderer class and append optional user-defined #class.
    let merged_class = utils::merge_class(classes::STYLED, parameters);
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        span
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            data-lk=[lk]
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
        let lk = styled
            .value()
            .attr("data-lk")
            .expect("styled span should have data-lk");
        let dk = styled
            .value()
            .attr("data-dk")
            .expect("styled span should have data-dk");
        let sheet = doc
            .select(&selector("style"))
            .next()
            .expect("expected shared stylesheet")
            .inner_html();

        assert!(
            styled.value().attr("style").is_none(),
            "light styles should be moved out of inline style attributes, got:\n{html}"
        );
        assert!(
            sheet.contains(&format!(r#"[data-lk="{lk}"]"#)),
            "expected shared stylesheet to contain the light rule, got:\n{html}"
        );
        assert!(
            sheet.contains("display: inline-block") && sheet.contains("color: red"),
            "expected allowed light style fragments to survive sanitization, got:\n{html}"
        );
        assert!(
            !sheet.contains("position"),
            "expected overlay positioning to be removed, got:\n{html}"
        );
        assert!(
            sheet.contains(&format!(r#".dark [data-dk="{dk}"]"#)),
            "expected shared stylesheet to contain the dark rule, got:\n{html}"
        );
        assert!(
            sheet.contains("display: grid") && sheet.contains("color: #00f"),
            "expected allowed dark style fragments to survive sanitization, got:\n{html}"
        );
        assert!(
            !sheet.contains("z-index"),
            "expected z-index to be removed, got:\n{html}"
        );
    }

    #[test]
    fn render_omits_style_keys_when_only_blocked_properties_remain() {
        let input = r#"{{{ #style="position:fixed; z-index:1" #dark-style="inset:0; pointer-events:none" Unsafe only }}}"#;
        let html = render_html(input);
        let doc = parse_fragment(&html);
        let styled = doc
            .select(&selector("span.sm-styled"))
            .next()
            .expect("expected styled span");

        assert!(
            styled.value().attr("style").is_none(),
            "expected inline style attribute to stay omitted, got:\n{html}"
        );
        assert!(
            styled.value().attr("data-lk").is_none(),
            "expected no light key when all light properties are blocked, got:\n{html}"
        );
        assert!(
            !html.contains("data-dk"),
            "expected no dark style tag when all dark properties are blocked, got:\n{html}"
        );
    }
}
