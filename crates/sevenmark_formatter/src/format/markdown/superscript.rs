use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Element;

use crate::format::element::format_elements;

pub fn format_superscript<'a>(
    a: &'a Arena<'a>,
    children: &[Element],
) -> DocBuilder<'a, Arena<'a>> {
    a.text("^^")
        .append(format_elements(a, children))
        .append(a.text("^^"))
}
