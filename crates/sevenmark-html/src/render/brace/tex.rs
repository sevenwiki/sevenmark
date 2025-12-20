//! TeX rendering

use maud::{Markup, html};
use sevenmark_parser::ast::TeXElement;

use crate::classes;

pub fn render(e: &TeXElement) -> Markup {
    if e.is_block {
        html! { div class=(format!("{} {}", classes::TEX, classes::TEX_BLOCK)) data-tex=(&e.content) { (&e.content) } }
    } else {
        html! { span class=(format!("{} {}", classes::TEX, classes::TEX_INLINE)) data-tex=(&e.content) { (&e.content) } }
    }
}
