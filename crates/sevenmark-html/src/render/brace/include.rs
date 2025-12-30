//! Include element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::IncludeElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(e: &IncludeElement, ctx: &mut RenderContext) -> Markup {
    html! {
        span class=(classes::INCLUDE) {
            (render_elements(&e.content, ctx))
        }
    }
}
