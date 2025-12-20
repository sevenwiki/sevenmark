//! Footnote rendering

use maud::{Markup, html};
use sevenmark_parser::ast::FootnoteElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

/// Render inline footnote reference
pub fn render(e: &FootnoteElement, ctx: &mut RenderContext) -> Markup {
    if ctx.in_footnote {
        // Prevent nested footnotes - just render content
        return render_elements(&e.content, ctx);
    }

    let index = e.footnote_index;
    let display = utils::get_param(&e.parameters, "display");
    let display_text = ctx.add_footnote(index, display, e.content.clone());

    html! {
        sup class=(classes::FOOTNOTE) id=(format!("{}{}", classes::FOOTNOTE_REF_ID_PREFIX, index)) {
            a class=(classes::FOOTNOTE_REF) href=(format!("#{}{}", classes::FOOTNOTE_ID_PREFIX, index)) {
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
    let mut inner_ctx = RenderContext::new();
    inner_ctx.in_footnote = true;

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
