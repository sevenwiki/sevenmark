use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{Element, FoldElement, FoldInnerElement};

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::params::{format_params_block, format_params_block_tight};

pub fn format_fold<'a>(
    a: &'a Arena<'a>,
    e: &FoldElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = " ".repeat(config.indent);
    let params = format_params_block(a, &e.parameters, config);
    let inner = a
        .hardline()
        .append(a.text(indent.clone()))
        .append(format_fold_inner(a, &e.summary, config))
        .append(a.hardline())
        .append(a.text(indent))
        .append(format_fold_inner(a, &e.details, config));
    a.text("{{{#fold")
        .append(params)
        .append(inner)
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_fold_inner<'a>(
    a: &'a Arena<'a>,
    inner: &FoldInnerElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &inner.parameters, config);
    let has_params = !inner.parameters.is_empty();
    let closing = if needs_line_break_before_inner_close(&inner.children) {
        a.hardline().append(a.text("]]"))
    } else {
        a.text("]]")
    };
    a.text("[[")
        .append(params)
        .append(if inner.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ")
                .append(format_elements(a, &inner.children, config))
        } else {
            format_elements(a, &inner.children, config)
        })
        .append(closing)
}

fn needs_line_break_before_inner_close(children: &[Element]) -> bool {
    let last_semantic = children
        .iter()
        .rev()
        .find(|el| !is_ignorable_trailing_text(el));

    matches!(
        last_semantic,
        Some(
            Element::Table(_)
                | Element::List(_)
                | Element::Fold(_)
                | Element::Code(_)
                | Element::TeX(_)
                | Element::Css(_)
                | Element::BlockQuote(_)
                | Element::Literal(_)
                | Element::Styled(_)
                | Element::Include(_)
                | Element::Footnote(_)
                | Element::If(_)
        )
    )
}

fn is_ignorable_trailing_text(el: &Element) -> bool {
    match el {
        Element::Text(t) => t.value.chars().all(|c| matches!(c, ' ' | '\t' | '\r')),
        Element::SoftBreak(_) => true,
        _ => false,
    }
}
