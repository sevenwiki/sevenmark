use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::IfElement;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};
use crate::format::expression::format_expr;

pub fn format_if<'a>(
    a: &'a Arena<'a>,
    e: &IfElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#if ")
        .append(format_expr(a, &e.condition, config))
        .append(a.text(" :: "))
        .append(format_elements_with_context(
            a,
            &e.children,
            config,
            context,
        ))
        .append(a.text("}}}"))
}
