use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{FoldElement, FoldInnerElement};

use crate::format::element::format_elements;
use crate::format::params::{format_params_block, format_params_block_tight};

const INDENT: isize = 2;

pub fn format_fold<'a>(a: &'a Arena<'a>, e: &FoldElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block(a, &e.parameters);
    let inner = a
        .hardline()
        .append(format_fold_inner(a, &e.summary))
        .append(a.hardline())
        .append(format_fold_inner(a, &e.details));
    a.text("{{{#fold")
        .append(params)
        .append(inner.nest(INDENT))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_fold_inner<'a>(a: &'a Arena<'a>, inner: &FoldInnerElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &inner.parameters);
    let has_params = !inner.parameters.is_empty();
    a.text("[[")
        .append(params)
        .append(if inner.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements(a, &inner.children))
        } else {
            format_elements(a, &inner.children)
        })
        .append(a.text("]]"))
}
