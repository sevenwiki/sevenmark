//! Escape element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::EscapeElement;

pub fn render(e: &EscapeElement) -> Markup {
    html! { (e.content) }
}
