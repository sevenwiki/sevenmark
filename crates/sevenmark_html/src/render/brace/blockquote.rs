//! BlockQuote rendering

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
    let content = render_elements(children, ctx);

    let lk = ctx.add_light_style(utils::build_style(parameters));
    let merged_class = utils::merge_class(classes::BLOCKQUOTE, parameters);
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        blockquote
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            data-lk=[lk]
            data-dk=[dk]
        { (content) }
    }
}
