//! Fold element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::FoldElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &FoldElement, ctx: &mut RenderContext) -> Markup {
    let summary = render_elements(&e.content.0.content, ctx);
    let details = render_elements(&e.content.1.content, ctx);
    let style = utils::build_style(&e.parameters);

    html! {
        details class=(classes::FOLD) style=[style] {
            summary class=(classes::FOLD_SUMMARY) { (summary) }
            (details)
        }
    }
}
