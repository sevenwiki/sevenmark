use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::MediaElement;

use crate::format::element::format_elements;
use crate::format::params::format_params_tight;

pub fn format_media<'a>(a: &'a Arena<'a>, e: &MediaElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("[[")
        .append(format_params_tight(a, &e.parameters))
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, &e.children))
        })
        .append(a.text("]]"))
}
