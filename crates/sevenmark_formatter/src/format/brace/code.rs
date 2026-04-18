use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CodeElement;

use crate::FormatConfig;
use crate::format::brace::common::needs_close_separator_for_raw_value;
use crate::format::params::format_params;

pub fn format_code<'a>(
    a: &'a Arena<'a>,
    e: &CodeElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let close_separator = if needs_close_separator_for_raw_value(&e.value) {
        a.text(" ")
    } else {
        a.nil()
    };

    a.text("{{{#code")
        .append(format_params(a, &e.parameters, config))
        .append(a.hardline())
        .append(a.text(e.value.clone()))
        .append(close_separator)
        .append(a.text("}}}"))
}
