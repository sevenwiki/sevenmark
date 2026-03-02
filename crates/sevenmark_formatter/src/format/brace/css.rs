use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::CssElement;

use crate::FormatConfig;
use crate::format::params::format_params;

pub fn format_css<'a>(
    a: &'a Arena<'a>,
    e: &CssElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#css")
        .append(format_params(a, &e.parameters, config))
        .append(a.hardline())
        .append(a.text(e.value.clone()))
        .append(a.text("}}}"))
}
