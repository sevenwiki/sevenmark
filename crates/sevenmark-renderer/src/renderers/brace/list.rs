//! List element renderer

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::{ListContentItem, ListElement};

/// Render list element
pub fn render_list(elem: &ListElement, ctx: &mut RenderContext) -> Markup {
    let items = render_list_items(&elem.content, ctx);

    match elem.kind.as_str() {
        "ol" | "1" => html! { ol { (items) } },
        "a" => html! { ol type="a" { (items) } },
        "A" => html! { ol type="A" { (items) } },
        "i" => html! { ol type="i" { (items) } },
        "I" => html! { ol type="I" { (items) } },
        _ => html! { ul { (items) } }, // default: unordered
    }
}

fn render_list_items(items: &[ListContentItem], ctx: &mut RenderContext) -> Markup {
    html! {
        @for item in items {
            (render_list_item(item, ctx))
        }
    }
}

fn render_list_item(item: &ListContentItem, ctx: &mut RenderContext) -> Markup {
    match item {
        ListContentItem::Item(inner) => {
            html! {
                li {
                    (render_elements(&inner.content, ctx))
                }
            }
        }
        ListContentItem::Conditional { items, .. } => {
            // Conditional items should be processed in transform
            html! {
                @for inner in items {
                    li {
                        (render_elements(&inner.content, ctx))
                    }
                }
            }
        }
    }
}
