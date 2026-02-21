use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{ConditionalListItems, ListContentItem, ListElement, ListItemElement};

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::expression::format_expr;
use crate::format::params::{format_params_block, format_params_block_tight};

pub fn format_list<'a>(
    a: &'a Arena<'a>,
    e: &ListElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let params = format_params_block(a, &e.parameters, config);
    let items = a.intersperse(
        e.children
            .iter()
            .map(|item| format_list_content_item(a, item, config)),
        a.hardline(),
    );
    a.text("{{{#list")
        .append(params)
        .append(a.hardline().append(items).nest(indent))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_list_content_item<'a>(
    a: &'a Arena<'a>,
    item: &ListContentItem,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    match item {
        ListContentItem::Item(li) => format_list_item(a, li, config),
        ListContentItem::Conditional(cond) => format_conditional_list_items(a, cond, config),
    }
}

fn format_list_item<'a>(
    a: &'a Arena<'a>,
    li: &ListItemElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &li.parameters, config);
    let has_params = !li.parameters.is_empty();
    a.text("[[")
        .append(params)
        .append(if li.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements(a, &li.children, config))
        } else {
            format_elements(a, &li.children, config)
        })
        .append(a.text("]]"))
}

fn format_conditional_list_items<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalListItems,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let items = a.intersperse(
        cond.items
            .iter()
            .map(|li| format_list_item(a, li, config)),
        a.hardline(),
    );
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition, config))
        .append(a.text(" ::"))
        .append(a.hardline().append(items).nest(indent))
        .append(a.hardline())
        .append(a.text("}}}"))
}
