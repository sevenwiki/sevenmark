use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{ConditionalListItems, Element, ListContentItem, ListElement, ListItemElement};

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
    let closing = if needs_line_break_before_item_close(&li.children) {
        a.hardline().append(a.text("]]"))
    } else {
        a.text("]]")
    };

    a.text("[[")
        .append(params)
        .append(if li.children.is_empty() {
            a.nil()
        } else if has_params {
            a.text(" ").append(format_elements(a, &li.children, config))
        } else {
            format_elements(a, &li.children, config)
        })
        .append(closing)
}

fn format_conditional_list_items<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalListItems,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let items = a.intersperse(
        cond.items.iter().map(|li| format_list_item(a, li, config)),
        a.hardline(),
    );
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition, config))
        .append(a.text(" ::"))
        .append(a.hardline().append(items).nest(indent))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn needs_line_break_before_item_close(children: &[Element]) -> bool {
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
