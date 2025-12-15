use maud::{html, Markup};
use sevenmark_parser::ast::TextStyle;
use crate::context::RenderContext;
use crate::render_elements;


/// Render subscript text
pub fn render_subscript(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        sub { (render_elements(&style.content, ctx)) }
    }
}
