//! Ruby element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{Element, Parameters, Span};

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
    let base = render_elements(children, ctx);
    ctx.exit_suppress_soft_breaks();

    let ruby_text = utils::get_param(parameters, "ruby").unwrap_or_default();
    let style = utils::build_style(parameters);

    html! {
        ruby
            class=(classes::RUBY)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
        {
            (base)
            rt { (ruby_text) }
        }
    }
}
