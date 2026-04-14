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
    ctx.enter_suppress_soft_breaks();
    let content = render_elements(children, ctx);
    ctx.exit_suppress_soft_breaks();

    let style = utils::build_style(parameters);
    let merged_class = utils::merge_class(classes::BLOCKQUOTE, parameters);
    let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(parameters));

    html! {
        (dark_tag)
        blockquote
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
            data-dk=[dk]
        { (content) }
    }
}
