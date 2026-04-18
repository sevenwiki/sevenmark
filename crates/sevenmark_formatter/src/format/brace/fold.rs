use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{FoldElement, FoldInnerElement};

use crate::FormatConfig;
use crate::format::element::{FormatContext, format_elements_with_context};
use crate::format::params::{format_params_block, format_params_block_tight};

pub fn format_fold<'a>(
    a: &'a Arena<'a>,
    e: &FoldElement,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    let fold_context = context.suppress_soft_breaks();
    let indent = " ".repeat(config.indent);
    let params = format_params_block(a, &e.parameters, config);
    let inner = a
        .hardline()
        .append(a.text(indent.clone()))
        .append(format_fold_inner(a, &e.summary, config, fold_context))
        .append(a.hardline())
        .append(a.text(indent))
        .append(format_fold_inner(a, &e.details, config, fold_context));
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
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &inner.parameters, config);
    let has_params = !inner.parameters.is_empty();
    a.text("[[")
        .append(params)
        .append(if inner.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements_with_context(
                a,
                &inner.children,
                config,
                context,
            ))
        } else {
            format_elements_with_context(a, &inner.children, config, context)
        })
        .append(a.text("]]"))
}
