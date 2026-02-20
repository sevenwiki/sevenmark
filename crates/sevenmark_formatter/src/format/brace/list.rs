use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{ConditionalListItems, ListContentItem, ListElement, ListItemElement};

use crate::format::element::format_elements;
use crate::format::expression::format_expr;
use crate::format::params::{format_params_block, format_params_block_tight};

const INDENT: isize = 2;

pub fn format_list<'a>(a: &'a Arena<'a>, e: &ListElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block(a, &e.parameters);
    // items: 한 줄에 들어가면 공백 구분, 넘으면 줄바꿈
    let items = a.intersperse(
        e.children.iter().map(|item| format_list_content_item(a, item)),
        a.line(),
    );
    a.text("{{{#list")
        .append(params)
        .append(a.hardline().append(items.group()).nest(INDENT))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_list_content_item<'a>(
    a: &'a Arena<'a>,
    item: &ListContentItem,
) -> DocBuilder<'a, Arena<'a>> {
    match item {
        ListContentItem::Item(li) => format_list_item(a, li),
        ListContentItem::Conditional(cond) => format_conditional_list_items(a, cond),
    }
}

fn format_list_item<'a>(a: &'a Arena<'a>, li: &ListItemElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &li.parameters);
    let has_params = !li.parameters.is_empty();
    a.text("[[")
        .append(params)
        .append(if li.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements(a, &li.children))
        } else {
            format_elements(a, &li.children)
        })
        .append(a.text("]]"))
}

fn format_conditional_list_items<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalListItems,
) -> DocBuilder<'a, Arena<'a>> {
    let items = a.intersperse(
        cond.items.iter().map(|li| format_list_item(a, li)),
        a.line(),
    );
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition))
        .append(a.text(" ::"))
        .append(a.line().append(items).nest(INDENT).group())
        .append(a.hardline())
        .append(a.text("}}}"))
}
