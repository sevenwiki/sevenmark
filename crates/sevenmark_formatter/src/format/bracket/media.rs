use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{Element, MediaElement};

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::params::format_params_tight;

pub fn format_media<'a>(
    a: &'a Arena<'a>,
    e: &MediaElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let has_params = !e.parameters.is_empty();
    let closing = if needs_line_break_before_media_close(&e.children) {
        a.hardline().append(a.text("]]"))
    } else {
        a.text("]]")
    };

    a.text("[[")
        .append(format_params_tight(a, &e.parameters, config))
        .append(if e.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements(a, &e.children, config))
        } else {
            format_elements(a, &e.children, config)
        })
        .append(closing)
}

fn needs_line_break_before_media_close(children: &[Element]) -> bool {
    let last_semantic = children
        .iter()
        .rev()
        .find(|el| !is_ignorable_trailing_text(el));

    matches!(
        last_semantic,
        Some(Element::Code(_) | Element::TeX(_) | Element::Css(_))
    )
}

fn is_ignorable_trailing_text(el: &Element) -> bool {
    match el {
        Element::Text(t) => t.value.chars().all(|c| matches!(c, ' ' | '\t' | '\r')),
        Element::SoftBreak(_) => true,
        _ => false,
    }
}
