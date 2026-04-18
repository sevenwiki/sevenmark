use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{BlockQuoteElement, Element};

use crate::FormatConfig;
use crate::format::brace::common::needs_close_separator_for_elements;
use crate::format::element::{
    FormatContext, TrailingSoftBreakPolicy, format_elements_with_context,
};
use crate::format::params::format_params_block;

pub fn format_blockquote<'a>(
    a: &'a Arena<'a>,
    e: &BlockQuoteElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    let trailing_soft_break_count = count_trailing_soft_breaks(&e.children);
    let semantic_children_end = e.children.len().saturating_sub(trailing_soft_break_count);
    let semantic_children = &e.children[..semantic_children_end];

    let close_separator = if needs_close_separator_for_elements(semantic_children) {
        a.text(" ")
    } else {
        a.nil()
    };

    let quote_context = context.with_trailing_soft_break_policy(TrailingSoftBreakPolicy::Drop);

    a.text("{{{#quote")
        .append(format_params_block(a, &e.parameters, config))
        .append(if e.children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements_with_context(
                a,
                &e.children,
                config,
                quote_context,
            ))
        })
        .append(close_separator)
        .append(a.text("}}}"))
}

fn count_trailing_soft_breaks(children: &[Element]) -> usize {
    children
        .iter()
        .rev()
        .take_while(|el| matches!(el, Element::SoftBreak(_)))
        .count()
}
