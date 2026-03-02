use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{FoldElement, FoldInnerElement};

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
        .append(a.text("]]"))
}
