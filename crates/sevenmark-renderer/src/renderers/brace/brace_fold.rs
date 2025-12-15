//! Fold (collapsible) element renderer

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::FoldElement;

/// Render fold element as details/summary
/// content is a tuple: (summary, content)
pub fn render_brace_fold(elem: &FoldElement, ctx: &mut RenderContext) -> Markup {
    let (summary, content) = &elem.content;

    html! {
        details class="sm-fold" {
            summary {
                (render_elements(&summary.content, ctx))
            }
            div class="sm-fold-content" {
                (render_elements(&content.content, ctx))
            }
        }
    }
}
