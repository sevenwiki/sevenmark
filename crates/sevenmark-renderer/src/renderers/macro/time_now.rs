//! TimeNow macro renderer

use crate::context::RenderContext;
use maud::{Markup, html};

/// Render [now()] macro - current time
pub fn render_time_now(ctx: &RenderContext) -> Markup {
    let formatted = ctx.now.format("%Y-%m-%d %H:%M:%S").to_string();
    html! { (formatted) }
}
