use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CssElement;

use crate::FormatConfig;
use crate::format::brace::raw::escape_line_only_closer;
use crate::format::params::format_params;

pub fn format_css<'a>(
    a: &'a Arena<'a>,
    e: &CssElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let value = escape_line_only_closer(&e.value, "}}}");

    let close_prefix = if value.ends_with('\n') {
        a.nil()
    } else {
        a.hardline()
    };

    a.text("{{{#css")
        .append(format_params(a, &e.parameters, config))
        .append(a.hardline())
        .append(a.text(value))
        .append(close_prefix)
        .append(a.text("}}}"))
}
