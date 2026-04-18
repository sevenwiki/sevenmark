use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{BlockQuoteElement, Element};

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::params::format_params_block;

pub fn format_blockquote<'a>(
    a: &'a Arena<'a>,
    e: &BlockQuoteElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let children = trim_trailing_quote_children(&e.children);
    a.text("{{{#quote")
        .append(format_params_block(a, &e.parameters, config))
        .append(if children.is_empty() {
            a.nil()
        } else {
            a.text(" ").append(format_elements(a, children, config))
        })
        .append(a.text("}}}"))
}

fn trim_trailing_quote_children(children: &[Element]) -> &[Element] {
    let end = children
        .iter()
        .rposition(|el| !is_ignorable_trailing(el))
        .map_or(0, |idx| idx + 1);
    &children[..end]
}

fn is_ignorable_trailing(el: &Element) -> bool {
    match el {
        Element::SoftBreak(_) => true,
        Element::Text(text) => text
            .value
            .chars()
            .all(|c| matches!(c, ' ' | '\t' | '\r' | '\n')),
        _ => false,
    }
}
