//! Include element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{Element, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(span: &Span, children: &[Element], ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let content = render_elements(children, ctx);
    ctx.exit_suppress_soft_breaks();

    html! {
        span
            class=(classes::INCLUDE)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        {
            (content)
        }
    }
}
