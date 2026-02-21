use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::ExternalMediaElement;

use crate::FormatConfig;
use crate::format::params::format_params;

pub fn format_external_media<'a>(
    a: &'a Arena<'a>,
    e: &ExternalMediaElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    a.text(format!("[[#{}", e.provider))
        .append(format_params(a, &e.parameters, config))
        .append(a.text("]]"))
}
