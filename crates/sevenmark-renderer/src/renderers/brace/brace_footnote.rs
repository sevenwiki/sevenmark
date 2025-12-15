//! Footnote element renderer

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::FootnoteElement;

/// Render footnote element
/// Adds footnote to context and returns a reference marker
pub fn render_brace_footnote(elem: &FootnoteElement, ctx: &mut RenderContext) -> Markup {
    let index = ctx.add_footnote(elem.content.clone());

    html! {
        sup class="sm-footnote-ref" {
            a id=(format!("fnref-{}", index)) href=(format!("#fn-{}", index)) {
                "[" (index) "]"
            }
        }
    }
}

/// Render collected footnotes section
/// Call this at the end of document rendering
pub fn render_footnotes_section(ctx: &mut RenderContext) -> Markup {
    if !ctx.has_footnotes() {
        return html! {};
    }

    // We need to collect footnotes and render them
    // Since footnotes contain SevenMarkElements, we need to render each one
    let footnotes: Vec<_> = ctx.footnotes().to_vec();

    html! {
        section class="sm-footnotes" {
            hr;
            ol {
                @for footnote in &footnotes {
                    li id=(format!("fn-{}", footnote.index)) {
                        (render_elements(&footnote.content, ctx))
                        " "
                        a href=(format!("#fnref-{}", footnote.index)) class="sm-footnote-backref" {
                            "↩"
                        }
                    }
                }
            }
        }
    }
}
