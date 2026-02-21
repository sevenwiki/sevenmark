use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::BlockQuoteElement;

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::params::format_params_block;

pub fn format_blockquote<'a>(
    a: &'a Arena<'a>,
    e: &BlockQuoteElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#quote")
        .append(format_params_block(a, &e.parameters, config))
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, &e.children, config))
        })
        .append(a.text("}}}"))
}
