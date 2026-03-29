//! FootNote macro rendering - renders collected footnotes at current position

use maud::{Markup, html};

use crate::context::RenderContext;
use crate::render::brace;

pub fn render(ctx: &mut RenderContext) -> Markup {
    if ctx.footnotes.is_empty() {
        return html! {};
    }

    // Take footnotes and render them at this position
    let footnotes = std::mem::take(&mut ctx.footnotes);
    brace::footnote::render_footnote_entries(&footnotes, ctx)
}
