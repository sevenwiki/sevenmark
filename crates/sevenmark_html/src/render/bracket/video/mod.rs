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
pub fn render(
    span: &Span,
    provider: &str,
    parameters: &Parameters,
    ctx: &mut RenderContext,
) -> Markup {
    let data_start = ctx.span_start(span);
    let data_end = ctx.span_end(span);

    match provider {
        "youtube" => youtube::render(data_start, data_end, parameters, ctx),
        "vimeo" => vimeo::render(data_start, data_end, parameters, ctx),
        "nicovideo" => nicovideo::render(data_start, data_end, parameters, ctx),
        "spotify" => spotify::render(data_start, data_end, parameters, ctx),
        "discord" => discord::render(data_start, data_end, parameters, ctx),
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
