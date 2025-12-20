//! Variable element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::VariableElement;

use crate::classes;

pub fn render(e: &VariableElement) -> Markup {
    html! { span class=(classes::VARIABLE) data-name=(&e.content) { (e.content) } }
}
