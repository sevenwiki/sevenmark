use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::StyledElement;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};
use crate::format::params::format_params_block;

pub fn format_styled<'a>(
    a: &'a Arena<'a>,
    e: &StyledElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{")
        .append(format_params_block(a, &e.parameters, config))
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
