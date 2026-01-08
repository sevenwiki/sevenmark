//! Video/Audio element rendering
//!
//! Supports: YouTube, Vimeo, NicoNico, Spotify

mod nicovideo;
mod spotify;
mod vimeo;
mod youtube;

use maud::{Markup, html};
use sevenmark_parser::ast::Parameters;

use crate::classes;

/// Render video/audio element by dispatching to provider-specific renderer
pub fn render(provider: &str, parameters: &Parameters) -> Markup {
    match provider {
        "youtube" => youtube::render(parameters),
        "vimeo" => vimeo::render(parameters),
        "nicovideo" => nicovideo::render(parameters),
        "spotify" => spotify::render(parameters),
        _ => html! {
            span class=(classes::ERROR) {
                "Unknown video provider: " (provider)
            }
        },
    }
}
