use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::DefineElement;

use crate::FormatConfig;
use crate::format::params::format_params;

pub fn format_define<'a>(
    a: &'a Arena<'a>,
    e: &DefineElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("{{{#define")
        .append(format_params(a, &e.parameters, config))
        .append(a.text("}}}"))
}
