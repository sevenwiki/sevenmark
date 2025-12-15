use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;

/// Render underline text
pub fn render_underline(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        u { (render_elements(&style.content, ctx)) }
    }
}