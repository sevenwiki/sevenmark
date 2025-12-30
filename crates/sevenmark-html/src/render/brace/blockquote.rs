//! BlockQuote rendering

use maud::{Markup, html};
use sevenmark_parser::ast::BlockQuoteElement;

use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &BlockQuoteElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let content = render_elements(&e.content, ctx);
    ctx.exit_suppress_soft_breaks();

    let style = utils::build_style(&e.parameters);

    html! { blockquote style=[style] { (content) } }
}
