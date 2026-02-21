use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::HeaderElement;

use crate::FormatConfig;
use crate::format::element::format_elements;

pub fn format_header<'a>(
    a: &'a Arena<'a>,
    e: &HeaderElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let hashes = "#".repeat(e.level);
    let prefix = if e.is_folded {
        format!("{}!", hashes)
    } else {
        hashes
    };
    a.text(prefix)
        .append(a.text(" "))
        .append(format_elements(a, &e.children, config))
}
