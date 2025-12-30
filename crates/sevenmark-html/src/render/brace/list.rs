//! List rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{ListContentItem, ListElement};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &ListElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let style = utils::build_style(&e.parameters);
    let items = render_items(&e.content, ctx);
    ctx.exit_suppress_soft_breaks();
    let is_ordered = !e.kind.is_empty();

    if is_ordered {
        // kind: "1", "a", "A", "i", "I"
        let list_type = if e.kind == "1" {
            None
        } else {
            Some(e.kind.as_str())
        };
        html! { ol class=(format!("{} {}", classes::LIST, classes::LIST_ORDERED)) type=[list_type] style=[style] { (items) } }
    } else {
        html! { ul class=(format!("{} {}", classes::LIST, classes::LIST_UNORDERED)) style=[style] { (items) } }
    }
}

fn render_items(items: &[ListContentItem], ctx: &mut RenderContext) -> Markup {
    html! {
        @for item in items {
            @match item {
                ListContentItem::Item(list_item) => {
                    @let style = utils::build_style(&list_item.parameters);
                    li style=[style] { (render_elements(&list_item.content, ctx)) }
                }
                ListContentItem::Conditional { items, .. } => {
                    @for list_item in items {
                        @let style = utils::build_style(&list_item.parameters);
                        li style=[style] { (render_elements(&list_item.content, ctx)) }
                    }
                }
            }
        }
    }
}
