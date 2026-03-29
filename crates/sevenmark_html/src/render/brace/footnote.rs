//! Footnote rendering

use maud::{Markup, html};
use sevenmark_ast::{Element, Parameters, Span};

use crate::classes;
use crate::context::{FootnoteEntry, RenderContext};
use crate::render::{render_elements, utils};

fn encode_named_footnote_fragment(name: &str) -> String {
    let mut encoded = String::with_capacity(name.len() * 2);
    for byte in name.bytes() {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded
}

fn named_footnote_id(name: &str) -> String {
    format!(
        "{}n-{}",
        classes::FOOTNOTE_ID_PREFIX,
        encode_named_footnote_fragment(name)
    )
}

fn named_footnote_ref_id(name: &str) -> String {
    format!(
        "{}n-{}",
        classes::FOOTNOTE_REF_ID_PREFIX,
        encode_named_footnote_fragment(name)
    )
}

fn duplicate_named_footnote_ref_id(name: &str, index: usize) -> String {
    format!("{}-{}", named_footnote_ref_id(name), index)
}

/// Generate footnote ID (for the footnote list item)
fn footnote_id(entry: &FootnoteEntry) -> String {
    match &entry.name {
        Some(name) => named_footnote_id(name),
        None => format!("{}{}", classes::FOOTNOTE_ID_PREFIX, entry.index),
    }
}

/// Generate footnote reference ID (for the inline sup element)
fn footnote_ref_id(entry: &FootnoteEntry) -> String {
    match &entry.name {
        Some(name) => named_footnote_ref_id(name),
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
                let ref_id = named_footnote_ref_id(&name);
                let fn_id = named_footnote_id(&name);
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
                let fn_id = named_footnote_id(&name);
                let ref_id = duplicate_named_footnote_ref_id(&name, footnote_index);
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

#[cfg(test)]
mod tests {
    use crate::{RenderConfig, render::render_document};
    use sevenmark_parser::core::parse_document;

    fn count_occurrences(haystack: &str, needle: &str) -> usize {
        haystack.match_indices(needle).count()
    }

    #[test]
    fn duplicate_named_footnote_refs_use_unique_ids() {
        let input = r#"A{{{#fn #name="a1" First A1. }}}.
B{{{#fn #name="a1" Duplicate A1. }}}.
C{{{#fn #name="a12" First A12. }}}.

[fn]"#;

        let ast = parse_document(input);
        let html = render_document(&ast, &RenderConfig::default());

        let first_a1_ref = super::named_footnote_ref_id("a1");
        let duplicate_a1_ref = super::duplicate_named_footnote_ref_id("a1", 2);
        let first_a12_ref = super::named_footnote_ref_id("a12");
        let a1_footnote_id = super::named_footnote_id("a1");

        assert_ne!(
            duplicate_a1_ref, first_a12_ref,
            "duplicate named footnote ref ids must not collide with other named refs"
        );
        assert!(
            html.contains(&format!("id=\"{}\"", first_a1_ref)),
            "expected first named reference id in output, got:\n{html}"
        );
        assert!(
            html.contains(&format!("id=\"{}\"", duplicate_a1_ref)),
            "expected duplicate named reference id in output, got:\n{html}"
        );
        assert!(
            html.contains(&format!("id=\"{}\"", first_a12_ref)),
            "expected second named reference id in output, got:\n{html}"
        );
        assert_eq!(
            count_occurrences(&html, &format!("id=\"{}\"", first_a12_ref)),
            1,
            "named reference ids should remain unique"
        );
        assert!(
            html.contains(&format!("href=\"#{}\"", a1_footnote_id)),
            "named references should still point at the original footnote entry"
        );
    }

    #[test]
    fn unnamed_footnotes_keep_contiguous_display_numbers_after_duplicate_named_refs() {
        let input = r#"A{{{#fn #name="a" First named. }}}.
B{{{#fn #name="a" Duplicate named. }}}.
C{{{#fn First unnamed. }}}.
D{{{#fn Second unnamed. }}}.

[fn]"#;

        let ast = parse_document(input);
        let html = render_document(&ast, &RenderConfig::default());

        assert!(
            html.contains(r##"id="rn3"><a class="sm-fn-ref" href="#fn3">[1]</a>"##),
            "expected first unnamed footnote to display as [1], got:\n{html}"
        );
        assert!(
            html.contains(r##"id="rn4"><a class="sm-fn-ref" href="#fn4">[2]</a>"##),
            "expected second unnamed footnote to display as [2], got:\n{html}"
        );
        assert!(
            !html.contains(r##"href="#fn3">[3]</a>"##),
            "duplicate named refs should not create visible numbering gaps, got:\n{html}"
        );
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
