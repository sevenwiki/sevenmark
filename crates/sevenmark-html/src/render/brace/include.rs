//! Include element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::IncludeElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(e: &IncludeElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let content = render_elements(&e.content, ctx);
    ctx.exit_suppress_soft_breaks();

    html! {
        span class=(classes::INCLUDE) {
            (content)
        }
    }
}
