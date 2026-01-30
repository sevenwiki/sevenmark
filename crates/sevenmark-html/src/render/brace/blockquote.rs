//! BlockQuote rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{Element, Parameters, Span};

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

    html! {
        blockquote
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
        { (content) }
    }
}
