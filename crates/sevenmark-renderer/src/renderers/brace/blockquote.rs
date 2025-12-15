//! BlockQuote renderer

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::BlockQuoteElement;

/// Render blockquote element
pub fn render_blockquote(elem: &BlockQuoteElement, ctx: &mut RenderContext) -> Markup {
    html! {
        blockquote {
            (render_elements(&elem.content, ctx))
        }
    }
}
