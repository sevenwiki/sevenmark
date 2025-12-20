//! Code block rendering

use maud::{Markup, html};
use sevenmark_parser::ast::CodeElement;

use crate::classes;
use crate::render::utils;

pub fn render(e: &CodeElement) -> Markup {
    let lang = utils::get_param(&e.parameters, "lang");
    html! {
        pre class=(classes::CODE) {
            code data-lang=[lang] { (&e.content) }
        }
    }
}
