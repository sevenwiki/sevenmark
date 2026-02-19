//! Variable element rendering

use maud::{Markup, html};
use sevenmark_ast::Span;

use crate::classes;
use crate::context::RenderContext;

pub fn render(span: &Span, name: &str, ctx: &RenderContext) -> Markup {
    html! {
        span
            class=(classes::VARIABLE)
            data-name=(name)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        { (name) }
    }
}
