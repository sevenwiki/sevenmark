use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CodeElement;

use crate::format::params::format_params;

pub fn format_code<'a>(a: &'a Arena<'a>, e: &CodeElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#code")
        .append(format_params(a, &e.parameters))
        .append(a.text(" "))
        .append(a.text(e.value.clone()))
        .append(a.text("}}}"))
}
