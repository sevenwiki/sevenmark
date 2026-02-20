use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CategoryElement;

use crate::format::element::format_elements;

pub fn format_category<'a>(a: &'a Arena<'a>, e: &CategoryElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#category")
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, &e.children))
        })
        .append(a.text("}}}"))
}
