use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::TeXElement;

pub fn format_tex<'a>(a: &'a Arena<'a>, e: &TeXElement) -> DocBuilder<'a, Arena<'a>> {
    let tag = if e.is_block {
        "{{{#tex #block"
    } else {
        "{{{#tex"
    };
    let close_separator = if e.value.ends_with('}') {
        a.text(" ")
    } else {
        a.nil()
    };

    a.text(tag)
        .append(a.hardline())
        .append(a.text(e.value.clone()))
        .append(close_separator)
        .append(a.text("}}}"))
}
