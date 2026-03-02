use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::LiteralElement;

use crate::FormatConfig;
use crate::format::brace::raw::needs_line_break_before_brace_close;
use crate::format::element::format_elements;

pub fn format_literal<'a>(
    a: &'a Arena<'a>,
    e: &LiteralElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let closing = if needs_line_break_before_brace_close(&e.children) {
        a.hardline().append(a.text("}}}"))
    } else {
        a.text("}}}")
    };

    a.text("{{{")
        .append(format_elements(a, &e.children, config))
        .append(closing)
}
