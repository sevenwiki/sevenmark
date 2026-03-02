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
    let dark_style = utils::build_dark_style(parameters);
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
            ol
                class=(merged_class)
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                type=[list_type]
                style=[style]
                data-dark-style=[dark_style]
            { (items) }
        }
    } else {
        let merged_class = utils::merge_class(
            &format!("{} {}", classes::LIST, classes::LIST_UNORDERED),
            parameters,
        );
        html! {
            ul
                class=(merged_class)
                data-start=[ctx.span_start(span)]
                data-end=[ctx.span_end(span)]
                style=[style]
                data-dark-style=[dark_style]
            { (items) }
        }
    }
}

fn render_items(items: &[ListContentItem], ctx: &mut RenderContext) -> Markup {
    html! {
        @for item in items {
            @match item {
                ListContentItem::Item(list_item) => {
                    @let style = utils::build_style(&list_item.parameters);
                    @let class = utils::param_class(&list_item.parameters);
                    @let dark_style = utils::build_dark_style(&list_item.parameters);
                    li class=[class] style=[style] data-dark-style=[dark_style] { (render_elements(&list_item.children, ctx)) }
                }
                ListContentItem::Conditional(cond) => {
                    @for list_item in &cond.items {
                        @let style = utils::build_style(&list_item.parameters);
                        @let class = utils::param_class(&list_item.parameters);
                        @let dark_style = utils::build_dark_style(&list_item.parameters);
                        li class=[class] style=[style] data-dark-style=[dark_style] { (render_elements(&list_item.children, ctx)) }
                    }
                }
            }
        }
    }
}
