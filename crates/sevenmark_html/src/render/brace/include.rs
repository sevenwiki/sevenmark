//! Include element rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(span: &Span, children: &[Element], ctx: &mut RenderContext) -> Markup {
    // Reset soft-break suppression so the included document renders naturally
    // (e.g. when {{{#include}}} appears inside {{{#fn}}}).
    let saved_depth = ctx.suppress_soft_breaks_depth;
    ctx.suppress_soft_breaks_depth = 0;
    let content = render_elements(children, ctx);
    ctx.suppress_soft_breaks_depth = saved_depth;

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
