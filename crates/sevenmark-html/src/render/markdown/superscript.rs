//! Superscript rendering

use maud::{Markup, html};
use sevenmark_parser::ast::TextStyle;

use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(e: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! { sup { (render_elements(&e.content, ctx)) } }
}
