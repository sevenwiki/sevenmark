use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;

/// Render bold text
pub fn render_bold(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        strong { (render_elements(&style.content, ctx)) }
    }
}