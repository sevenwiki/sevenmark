//! Footnote rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

/// Render inline footnote reference
pub fn render(
    span: &Span,
    footnote_index: usize,
    parameters: &Parameters,
    children: &[Element],
    ctx: &mut RenderContext,
) -> Markup {
    if ctx.in_footnote {
        // Prevent nested footnotes - just render content
        ctx.enter_suppress_soft_breaks();
        let content = render_elements(children, ctx);
        ctx.exit_suppress_soft_breaks();
        return content;
    }

    let data_start = ctx.span_start(span);
    let data_end = ctx.span_end(span);
    let display = utils::get_param(parameters, "display");
    let display_text = ctx.add_footnote(footnote_index, display, children.to_vec());

    html! {
        sup
            class=(classes::FOOTNOTE)
            data-start=[data_start]
            data-end=[data_end]
            id=(format!("{}{}", classes::FOOTNOTE_REF_ID_PREFIX, footnote_index))
        {
            a class=(classes::FOOTNOTE_REF) href=(format!("#{}{}", classes::FOOTNOTE_ID_PREFIX, footnote_index)) {
                "[" (display_text) "]"
            }
        }
    }
}

/// Render footnote list at document end
pub fn render_list(ctx: &RenderContext) -> Markup {
    if ctx.footnotes.is_empty() {
        return html! {};
    }

    // Render footnote contents with in_footnote flag set
    let mut inner_ctx = ctx.child();
    inner_ctx.in_footnote = true;
    inner_ctx.enter_suppress_soft_breaks();

    html! {
        section class=(classes::FOOTNOTE_LIST) {
            ol {
                @for entry in &ctx.footnotes {
                    li id=(format!("{}{}", classes::FOOTNOTE_ID_PREFIX, entry.index)) {
                        a class=(classes::FOOTNOTE_BACK) href=(format!("#{}{}", classes::FOOTNOTE_REF_ID_PREFIX, entry.index)) {
                            "[" (entry.display) "]"
                        }
                        " "
                        span class=(classes::FOOTNOTE_CONTENT) {
                            (render_elements(&entry.content, &mut inner_ctx))
                        }
                    }
                }
            }
        }
    }
}
