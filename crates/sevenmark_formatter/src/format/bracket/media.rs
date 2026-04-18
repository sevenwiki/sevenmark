use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::MediaElement;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};
use crate::format::params::format_params_tight;

pub fn format_media<'a>(
    a: &'a Arena<'a>,
    e: &MediaElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    let has_params = !e.parameters.is_empty();
    a.text("[[")
        .append(format_params_tight(a, &e.parameters, config))
        .append(if e.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ")
                .append(format_elements_with_context(a, &e.children, config, context))
        } else {
            format_elements_with_context(a, &e.children, config, context)
        })
        .append(a.text("]]"))
}
