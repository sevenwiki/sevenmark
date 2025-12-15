use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;


/// Render strikethrough text
pub fn render_strikethrough(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        del { (render_elements(&style.content, ctx)) }
    }
}

