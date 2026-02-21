use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Element;

use crate::FormatConfig;
use crate::format::element::format_elements;

pub fn format_subscript<'a>(
    a: &'a Arena<'a>,
    children: &[Element],
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text(",,")
        .append(format_elements(a, children, config))
        .append(a.text(",,"))
}
