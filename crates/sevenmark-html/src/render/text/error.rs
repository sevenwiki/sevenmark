//! Error element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::ErrorElement;

use crate::classes;

pub fn render(e: &ErrorElement) -> Markup {
    html! { span class=(classes::ERROR) { (e.content) } }
}
