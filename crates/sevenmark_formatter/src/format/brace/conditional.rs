use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::IfElement;

use crate::format::element::format_elements;
use crate::format::expression::format_expr;

pub fn format_if<'a>(a: &'a Arena<'a>, e: &IfElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#if ")
        .append(format_expr(a, &e.condition))
        .append(a.text(" :: "))
        .append(format_elements(a, &e.children))
        .append(a.text("}}}"))
}
