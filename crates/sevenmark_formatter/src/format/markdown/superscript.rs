use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Element;

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};

pub fn format_superscript<'a>(
    a: &'a Arena<'a>,
    children: &[Element],
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    a.text("^^")
        .append(format_elements_with_context(a, children, config, context))
        .append(a.text("^^"))
}
