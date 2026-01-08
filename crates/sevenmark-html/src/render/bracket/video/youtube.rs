//! YouTube video rendering (direct iframe embed)
//!
//! Embed URL: https://www.youtube.com/embed/{id}
//!
//! Parameters:
//!   - id: Video ID (required)
//!   - width, height: Dimensions (default: 640x360)
//!   - start: Start time in seconds
//!   - end: End time in seconds
//!   - autoplay: Auto-play on load
//!   - loop: Loop video
//!   - mute: Start muted
//!   - controls: Show controls (default: true)

use maud::{Markup, html};
use sevenmark_parser::ast::Parameters;

use crate::classes;
use crate::render::utils::get_param;

fn build_embed_url(id: &str, parameters: &Parameters) -> String {
    let mut params = Vec::new();

    if let Some(start) = get_param(parameters, "start") {
        params.push(format!("start={}", start));
    }
    if let Some(end) = get_param(parameters, "end") {
        params.push(format!("end={}", end));
    }
    if get_param(parameters, "autoplay").is_some() {
        params.push("autoplay=1".to_string());
    }
    if get_param(parameters, "loop").is_some() {
        params.push("loop=1".to_string());
        params.push(format!("playlist={}", id));
    }
    if get_param(parameters, "mute").is_some() {
        params.push("mute=1".to_string());
    }
    if let Some(controls) = get_param(parameters, "controls") {
        if controls == "0" || controls == "false" {
            params.push("controls=0".to_string());
        }
    }

    if params.is_empty() {
        format!("https://www.youtube.com/embed/{}", id)
    } else {
        format!("https://www.youtube.com/embed/{}?{}", id, params.join("&"))
    }
}

pub fn render(parameters: &Parameters) -> Markup {
    let id = match get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) { "YouTube: missing id parameter" }
            }
        }
    };

    let url = build_embed_url(&id, parameters);
    let width = get_param(parameters, "width").unwrap_or_else(|| "640".to_string());
    let height = get_param(parameters, "height").unwrap_or_else(|| "360".to_string());

    html! {
        iframe
            class=(format!("{} {}", classes::VIDEO, classes::VIDEO_YOUTUBE))
            src=(url)
            width=(width)
            height=(height)
            frameborder="0"
            allowfullscreen
            loading="lazy"
        {}
    }
}