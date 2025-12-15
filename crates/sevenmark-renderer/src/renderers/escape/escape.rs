use maud::{html, Markup};
use sevenmark_parser::ast::EscapeElement;

/// Render escaped character (already escaped in source)
pub fn render_escape(elem: &EscapeElement) -> Markup {
    html! { (elem.content) }
}