use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::FootnoteElement;

use crate::format::element::format_elements;
use crate::format::params::format_params_block;

pub fn format_footnote<'a>(a: &'a Arena<'a>, e: &FootnoteElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#fn")
        .append(format_params_block(a, &e.parameters))
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, &e.children))
        })
        .append(a.text("}}}"))
}
