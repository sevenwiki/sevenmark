//! Literal element rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(span: &Span, children: &[Element], ctx: &mut RenderContext) -> Markup {
    html! {
        pre
            class=(classes::LITERAL)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        { (render_elements(children, ctx)) }
    }
}
