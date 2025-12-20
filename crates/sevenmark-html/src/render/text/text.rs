//! Text element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::TextElement;

pub fn render(e: &TextElement) -> Markup {
    html! { (e.content) }
}
