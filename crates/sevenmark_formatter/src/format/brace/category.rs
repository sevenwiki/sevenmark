use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CategoryElement;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};

pub fn format_category<'a>(
    a: &'a Arena<'a>,
    e: &CategoryElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#category")
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements_with_context(
                a,
                &e.children,
                config,
                context,
            ))
        })
        .append(a.text("}}}"))
}
