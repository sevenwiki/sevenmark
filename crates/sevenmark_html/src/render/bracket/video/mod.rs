//! External media element rendering
//!
//! Supports: YouTube, Vimeo, NicoNico, Spotify, Discord

mod discord;
mod nicovideo;
mod spotify;
mod vimeo;
mod youtube;

use maud::{Markup, html};
use sevenmark_ast::{Parameters, Span};

use crate::classes;
use crate::context::RenderContext;

/// Render external media element by dispatching to provider-specific renderer
pub fn render(span: &Span, provider: &str, parameters: &Parameters, ctx: &RenderContext) -> Markup {
    let data_start = ctx.span_start(span);
    let data_end = ctx.span_end(span);

    match provider {
        "youtube" => youtube::render(data_start, data_end, parameters),
        "vimeo" => vimeo::render(data_start, data_end, parameters),
        "nicovideo" => nicovideo::render(data_start, data_end, parameters),
        "spotify" => spotify::render(data_start, data_end, parameters),
        "discord" => discord::render(data_start, data_end, parameters),
        _ => html! {
            span
                class=(classes::ERROR)
                data-start=[data_start]
                data-end=[data_end]
            {
                "Unknown external media provider: " (provider)
            }
        },
    }
}
