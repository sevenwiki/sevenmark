use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::LiteralElement;

use crate::format::element::format_elements;

pub fn format_literal<'a>(a: &'a Arena<'a>, e: &LiteralElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{")
        .append(format_elements(a, &e.children))
        .append(a.text("}}}"))
}
