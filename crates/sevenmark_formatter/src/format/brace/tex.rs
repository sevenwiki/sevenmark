use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::TeXElement;

pub fn format_tex<'a>(a: &'a Arena<'a>, e: &TeXElement) -> DocBuilder<'a, Arena<'a>> {
    let tag = if e.is_block {
        "{{{#tex #block"
    } else {
        "{{{#tex"
    };
    a.text(tag)
        .append(a.text(" "))
        .append(a.text(e.value.clone()))
        .append(a.text("}}}"))
}
