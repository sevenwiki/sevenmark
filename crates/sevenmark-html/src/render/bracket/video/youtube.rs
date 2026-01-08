//! YouTube video rendering (facade pattern)
//!
//! Renders a thumbnail with play button. Client-side JS converts to iframe on click.
//! Thumbnail URL: https://i.ytimg.com/vi/{id}/hqdefault.jpg
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

pub fn render(parameters: &Parameters) -> Markup {
    let id = match get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) { "YouTube: missing id parameter" }
            }
        }
    };

    let thumbnail_url = format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", id);
    let width = get_param(parameters, "width").unwrap_or_else(|| "640".to_string());
    let height = get_param(parameters, "height").unwrap_or_else(|| "360".to_string());

    let start = get_param(parameters, "start");
    let end = get_param(parameters, "end");
    let autoplay = get_param(parameters, "autoplay").is_some();
    let loop_video = get_param(parameters, "loop").is_some();
    let mute = get_param(parameters, "mute").is_some();
    let controls_off = get_param(parameters, "controls")
        .map(|v| v == "0" || v == "false")
        .unwrap_or(false);

    html! {
        div
            class=(format!("{} {}", classes::VIDEO, classes::VIDEO_YOUTUBE))
            data-id=(id)
            data-width=(width)
            data-height=(height)
            data-start=[start]
            data-end=[end]
            data-autoplay[autoplay]
            data-loop[loop_video]
            data-mute[mute]
            data-controls-off[controls_off]
        {
            img
                class=(classes::VIDEO_THUMBNAIL)
                src=(thumbnail_url)
                alt="YouTube video thumbnail"
                loading="lazy"
            {}
            button class=(classes::VIDEO_PLAY_BUTTON) type="button" aria-label="Play video" {}
        }
    }
}