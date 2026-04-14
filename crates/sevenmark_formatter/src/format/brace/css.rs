use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CssElement;

use crate::FormatConfig;

pub fn format_css<'a>(
    a: &'a Arena<'a>,
    e: &CssElement,
    _config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let close_separator = if e.value.ends_with('}') {
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
