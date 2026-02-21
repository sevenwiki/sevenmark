use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::RedirectElement;

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::params::format_params;

pub fn format_redirect<'a>(
    a: &'a Arena<'a>,
    e: &RedirectElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#redirect")
        .append(format_params(a, &e.parameters, config))
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, &e.children, config))
        })
        .append(a.text("}}}"))
}
