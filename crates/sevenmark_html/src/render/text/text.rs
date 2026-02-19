//! Text element rendering

use maud::{Markup, html};
use sevenmark_ast::Span;

use crate::context::RenderContext;

pub fn render(span: &Span, value: &str, ctx: &RenderContext) -> Markup {
    html! {
        span data-start=[ctx.span_start(span)] data-end=[ctx.span_end(span)] { (value) }
    }
}
