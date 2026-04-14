//! List rendering

use maud::{Markup, html};
use sevenmark_ast::{ListContentItem, Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(
    span: &Span,
    kind: &str,
    parameters: &Parameters,
    children: &[ListContentItem],
    ctx: &mut RenderContext,
) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let style = utils::build_style(parameters);
    let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(parameters));

    // Collect dark style tags for all list items up front so they can be
    // emitted before the list container — injecting <style> inside <ul>/<ol>
    // would corrupt list-item position selectors (:first-child, :nth-child).
    let item_dark_tags = collect_item_dark_tags(children);

    let items = render_items(children, ctx);
    ctx.exit_suppress_soft_breaks();
    let is_ordered = !kind.is_empty();

    if is_ordered {
        // kind: "1", "a", "A", "i", "I"
        let list_type = if kind == "1" { None } else { Some(kind) };
        let merged_class = utils::merge_class(
            &format!("{} {}", classes::LIST, classes::LIST_ORDERED),
            parameters,
        );
        html! {
            (dark_tag)
            (item_dark_tags)
            ol
                class=(merged_class)
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                type=[list_type]
                style=[style]
                data-dk=[dk]
            { (items) }
        }
    } else {
        let merged_class = utils::merge_class(
            &format!("{} {}", classes::LIST, classes::LIST_UNORDERED),
            parameters,
        );
        html! {
            (dark_tag)
            (item_dark_tags)
            ul
                class=(merged_class)
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                style=[style]
                data-dk=[dk]
            { (items) }
        }
    }
}

/// Collect all dark-style `<style>` tags for list items (both direct and
/// conditional). Called before the list container is rendered so the tags
/// land outside any list structural element.
fn collect_item_dark_tags(items: &[ListContentItem]) -> Markup {
    let tags: Vec<Markup> = items
        .iter()
        .flat_map(|item| match item {
            ListContentItem::Item(list_item) => {
                vec![utils::dark_style_parts(utils::build_dark_style(&list_item.parameters)).1]
            }
            ListContentItem::Conditional(cond) => cond
                .items
                .iter()
                .map(|li| utils::dark_style_parts(utils::build_dark_style(&li.parameters)).1)
                .collect(),
        })
        .collect();
    html! { @for t in &tags { (t) } }
}

fn render_items(items: &[ListContentItem], ctx: &mut RenderContext) -> Markup {
    html! {
        @for item in items {
            @match item {
                ListContentItem::Item(list_item) => {
                    @let style = utils::build_style(&list_item.parameters);
                    @let class = utils::param_class(&list_item.parameters);
                    @let (dk, _) = utils::dark_style_parts(utils::build_dark_style(&list_item.parameters));
                    li class=[class] style=[style] data-dk=[dk] { (render_elements(&list_item.children, ctx)) }
                }
                ListContentItem::Conditional(cond) => {
                    @for list_item in &cond.items {
                        @let style = utils::build_style(&list_item.parameters);
                        @let class = utils::param_class(&list_item.parameters);
                        @let (dk, _) = utils::dark_style_parts(utils::build_dark_style(&list_item.parameters));
                        li class=[class] style=[style] data-dk=[dk] { (render_elements(&list_item.children, ctx)) }
                    }
                }
            }
        }
    }
}

