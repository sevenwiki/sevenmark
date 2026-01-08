//! NicoNico (nicovideo) video rendering (facade pattern)
//!
//! Renders a thumbnail with play button. Client-side JS converts to iframe on click.
//! Thumbnail URL: https://nicovideo.cdn.nimg.jp/thumbnails/{numeric_id}/{numeric_id}.L
//! Embed URL: https://embed.nicovideo.jp/watch/{id}
//!
//! Parameters:
//!   - id: Video ID (required, e.g., "sm9", "so39402840")
//!   - width, height: Dimensions (default: 640x360)
//!   - from: Start time in seconds
//!   - autoplay: Auto-play (set to "0" to disable, default enabled)

use maud::{Markup, html};
use sevenmark_parser::ast::Parameters;

use crate::classes;
use crate::render::utils::get_param;

/// Extract numeric ID from NicoNico video ID (e.g., "sm32921516" -> "32921516")
fn extract_numeric_id(id: &str) -> &str {
    id.trim_start_matches(|c: char| c.is_ascii_alphabetic())
}

pub fn render(parameters: &Parameters) -> Markup {
    let id = match get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) { "NicoNico: missing id parameter" }
            }
        }
    };

    let numeric_id = extract_numeric_id(&id);
    let thumbnail_url = format!(
        "https://nicovideo.cdn.nimg.jp/thumbnails/{}/{}.L",
        numeric_id, numeric_id
    );
    let width = get_param(parameters, "width").unwrap_or_else(|| "640".to_string());
    let height = get_param(parameters, "height").unwrap_or_else(|| "360".to_string());

    let from = get_param(parameters, "from");
    let autoplay_off = get_param(parameters, "autoplay")
        .map(|v| v == "0" || v == "false")
        .unwrap_or(false);

    html! {
        div
            class=(format!("{} {}", classes::VIDEO, classes::VIDEO_NICOVIDEO))
            data-id=(id)
            data-width=(width)
            data-height=(height)
            data-from=[from]
            data-autoplay-off[autoplay_off]
        {
            img
                class=(classes::VIDEO_THUMBNAIL)
                src=(thumbnail_url)
                alt="NicoNico video thumbnail"
                loading="lazy"
            {}
            button class=(classes::VIDEO_PLAY_BUTTON) type="button" aria-label="Play video" {}
        }
    }
}