use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::IfElement;

use crate::FormatConfig;
use crate::format::brace::raw::needs_line_break_before_brace_close;
use crate::format::element::format_elements;
use crate::format::expression::format_expr;

pub fn format_if<'a>(
    a: &'a Arena<'a>,
    e: &IfElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let closing = if needs_line_break_before_brace_close(&e.children) {
        a.hardline().append(a.text("}}}"))
    } else {
        a.text("}}}")
    };

    a.text("{{{#if ")
        .append(format_expr(a, &e.condition, config))
        .append(a.text(" :: "))
        .append(format_elements(a, &e.children, config))
        .append(closing)
}
