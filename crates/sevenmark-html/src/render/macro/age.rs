//! Age macro rendering

use maud::{Markup, html};
use sevenmark_parser::ast::AgeElement;

use crate::classes;

pub fn render(e: &AgeElement) -> Markup {
    html! { span class=(classes::AGE) data-date=(&e.content) {} }
}
