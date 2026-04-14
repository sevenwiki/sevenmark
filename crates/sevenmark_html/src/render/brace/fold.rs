//! Fold element rendering

use maud::{Markup, html};
use sevenmark_ast::FoldElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(fold: &FoldElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();

    let summary = render_elements(&fold.summary.children, ctx);
    let details = render_elements(&fold.details.children, ctx);
    ctx.exit_suppress_soft_breaks();

    let lk = ctx.add_light_style(utils::build_style(&fold.parameters));
    let merged_class = utils::merge_class(classes::FOLD, &fold.parameters);
    let dk = ctx.add_dark_style(utils::build_dark_style(&fold.parameters));

    html! {
        details
            class=(merged_class)
            data-start=[ctx.span_start(&fold.span)]
            data-end=[ctx.span_end(&fold.span)]
            data-lk=[lk]
            data-dk=[dk]
        {
            summary class=(classes::FOLD_SUMMARY) { (summary) }
            (details)
        }
    }
}
