use maud::{html, Markup};
use sevenmark_parser::ast::TextElement;

/// Render plain text (auto-escaped by maud)
pub fn render_text(elem: &TextElement) -> Markup {
    html! { (elem.content) }
}