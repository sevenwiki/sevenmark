//! TeX rendering

use maud::{Markup, html};
use sevenmark_parser::ast::Span;

use crate::classes;
use crate::context::RenderContext;

pub fn render(span: &Span, is_block: bool, value: &str, ctx: &RenderContext) -> Markup {
    if is_block {
        html! {
            div
                class=(format!("{} {}", classes::TEX, classes::TEX_BLOCK))
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                data-tex=(value)
            { (value) }
        }
    } else {
        html! {
            span
                class=(format!("{} {}", classes::TEX, classes::TEX_INLINE))
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                data-tex=(value)
            { (value) }
        }
    }
}
