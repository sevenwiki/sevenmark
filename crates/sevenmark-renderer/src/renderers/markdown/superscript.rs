use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;

/// Render superscript text
pub fn render_superscript(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        sup { (render_elements(&style.content, ctx)) }
    }
}