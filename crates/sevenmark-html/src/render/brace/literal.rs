//! Literal element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::LiteralElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(e: &LiteralElement, ctx: &mut RenderContext) -> Markup {
    html! { pre class=(classes::LITERAL) { (render_elements(&e.content, ctx)) } }
}
