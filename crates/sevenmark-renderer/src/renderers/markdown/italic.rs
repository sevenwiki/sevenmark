use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;

/// Render italic text
pub fn render_italic(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        em { (render_elements(&style.content, ctx)) }
    }
}