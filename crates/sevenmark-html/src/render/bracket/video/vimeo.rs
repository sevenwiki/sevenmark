//! Vimeo video rendering (facade pattern)
//!
//! Renders a placeholder with play button. Client-side JS fetches oEmbed for thumbnail.
//! oEmbed URL: https://vimeo.com/api/oembed.json?url=https://vimeo.com/{id}
//! Embed URL: https://player.vimeo.com/video/{id}
//!
//! Parameters:
//!   - id: Video ID (required)
//!   - h: Hash for unlisted videos
//!   - width, height: Dimensions (default: 640x360)
//!   - autoplay: Auto-play on load
//!   - loop: Loop video
//!   - mute: Start muted
//!   - color: Player accent color (hex without #)
//!   - dnt: Do Not Track mode

use maud::{Markup, html};
use sevenmark_parser::ast::Parameters;

use crate::classes;
use crate::render::utils::get_param;

pub fn render(parameters: &Parameters) -> Markup {
    let id = match get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) { "Vimeo: missing id parameter" }
            }
        }
    };

    let width = get_param(parameters, "width").unwrap_or_else(|| "640".to_string());
    let height = get_param(parameters, "height").unwrap_or_else(|| "360".to_string());

    let h = get_param(parameters, "h");
    let autoplay = get_param(parameters, "autoplay").is_some();
    let loop_video = get_param(parameters, "loop").is_some();
    let mute = get_param(parameters, "mute").is_some();
    let color = get_param(parameters, "color").map(|c| c.trim_start_matches('#').to_string());
    let dnt = get_param(parameters, "dnt").is_some();

    // Vimeo thumbnail requires oEmbed fetch, so we render a placeholder
    // Client-side JS will fetch thumbnail and update the img src
    html! {
        div
            class=(format!("{} {}", classes::VIDEO, classes::VIDEO_VIMEO))
            data-id=(id)
            data-width=(width)
            data-height=(height)
            data-h=[h]
            data-autoplay[autoplay]
            data-loop[loop_video]
            data-mute[mute]
            data-color=[color]
            data-dnt[dnt]
        {
            // Placeholder image (will be updated by client-side JS via oEmbed)
            img
                class=(classes::VIDEO_THUMBNAIL)
                src=""
                alt="Vimeo video thumbnail"
                loading="lazy"
                style="display:none"
            {}
            button class=(classes::VIDEO_PLAY_BUTTON) type="button" aria-label="Play video" {}
        }
    }
}
