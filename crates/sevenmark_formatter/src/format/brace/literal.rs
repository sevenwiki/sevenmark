use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::LiteralElement;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};

pub fn format_literal<'a>(
    a: &'a Arena<'a>,
    e: &LiteralElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{")
        .append(format_elements_with_context(
            a,
            &e.children,
            config,
            context,
        ))
        .append(a.text("}}}"))
}
