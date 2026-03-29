//! Footnote rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Parameters, Span};

use crate::classes;
use crate::context::{FootnoteEntry, RenderContext};
use crate::render::{render_elements, utils};

/// Generate footnote ID (for the footnote list item)
fn footnote_id(entry: &FootnoteEntry) -> String {
    match &entry.name {
        Some(name) => format!("{}{}", classes::FOOTNOTE_ID_PREFIX, name),
        None => format!("{}{}", classes::FOOTNOTE_ID_PREFIX, entry.index),
    }
}

/// Generate footnote reference ID (for the inline sup element)
fn footnote_ref_id(entry: &FootnoteEntry) -> String {
    match &entry.name {
        Some(name) => format!("{}{}", classes::FOOTNOTE_REF_ID_PREFIX, name),
        None => format!("{}{}", classes::FOOTNOTE_REF_ID_PREFIX, entry.index),
    }
}

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
    let name = utils::get_param(parameters, "name");

    // Named footnote path
    if let Some(name) = name {
        match ctx.add_named_footnote(footnote_index, name.clone(), children.to_vec()) {
            Ok(display_text) => {
                // First occurrence — create footnote entry
                let ref_id = format!("{}{}", classes::FOOTNOTE_REF_ID_PREFIX, name);
                let fn_id = format!("{}{}", classes::FOOTNOTE_ID_PREFIX, name);
                return html! {
                    sup
                        class=(classes::FOOTNOTE)
                        data-start=[data_start]
                        data-end=[data_end]
                        id=(ref_id)
                    {
                        a class=(classes::FOOTNOTE_REF) href=(format!("#{}", fn_id)) {
                            "[" (display_text) "]"
                        }
                    }
                };
            }
            Err(existing_index) => {
                // Duplicate — render as back-reference to existing footnote
                let fn_id = format!("{}{}", classes::FOOTNOTE_ID_PREFIX, name);
                let ref_id = format!(
                    "{}{}{}",
                    classes::FOOTNOTE_REF_ID_PREFIX,
                    name,
                    footnote_index
                );
                let _ = existing_index;
                return html! {
                    sup
                        class=(classes::FOOTNOTE)
                        data-start=[data_start]
                        data-end=[data_end]
                        id=(ref_id)
                    {
                        a class=(classes::FOOTNOTE_REF) href=(format!("#{}", fn_id)) {
                            "[" (name) "]"
                        }
                    }
                };
            }
        }
    }

    // Unnamed footnote — existing behavior
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

/// Render footnote list (used at document end and for mid-flush)
pub fn render_list(ctx: &RenderContext) -> Markup {
    render_footnote_entries(&ctx.footnotes, ctx)
}

/// Render a list of footnote entries
pub fn render_footnote_entries(entries: &[FootnoteEntry], ctx: &RenderContext) -> Markup {
    if entries.is_empty() {
        return html! {};
    }

    let mut inner_ctx = ctx.child();
    inner_ctx.in_footnote = true;
    inner_ctx.enter_suppress_soft_breaks();

    html! {
        section class=(classes::FOOTNOTE_LIST) {
            ol {
                @for entry in entries {
                    li id=(footnote_id(entry)) {
                        a class=(classes::FOOTNOTE_BACK) href=(format!("#{}", footnote_ref_id(entry))) {
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
