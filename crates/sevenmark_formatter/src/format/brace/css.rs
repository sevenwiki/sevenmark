use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CssElement;

use crate::FormatConfig;
use crate::format::brace::common::needs_close_separator_for_raw_value;

pub fn format_css<'a>(
    a: &'a Arena<'a>,
    e: &CssElement,
    _config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let close_separator = if needs_close_separator_for_raw_value(&e.value) {
        a.text(" ")
    } else {
        a.nil()
    };

    a.text("{{{#css")
        .append(a.hardline())
        .append(a.text(e.value.clone()))
        .append(close_separator)
        .append(a.text("}}}"))
}
