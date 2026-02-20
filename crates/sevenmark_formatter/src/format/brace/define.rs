use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::DefineElement;

use crate::format::params::format_params;

pub fn format_define<'a>(a: &'a Arena<'a>, e: &DefineElement) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#define")
        .append(format_params(a, &e.parameters))
        .append(a.text("}}}"))
}
