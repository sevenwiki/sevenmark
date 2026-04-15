//! Spotify audio/podcast rendering (direct iframe embed)
//!
//! Embed URL: https://open.spotify.com/embed/{type}/{id}
//!
//! Parameters:
//!   - track: Track ID
//!   - album: Album ID
//!   - playlist: Playlist ID
//!   - artist: Artist ID
//!   - episode: Podcast episode ID
//!   - show: Podcast show ID
//!   - width, height: Dimensions (overrides CSS default via data-lk)
//!   - dark: Dark theme (presence = enabled)
//!   - compact: Compact cover art view (presence = enabled)
//!
//! Note: One of track/album/playlist/artist/episode/show is required

use maud::{Markup, html};
use sevenmark_ast::Parameters;

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

fn get_content_type_and_id(parameters: &Parameters) -> Option<(&'static str, String)> {
    if let Some(id) = utils::get_param(parameters, "track") {
        return Some(("track", id));
    }
    if let Some(id) = utils::get_param(parameters, "album") {
        return Some(("album", id));
    }
    if let Some(id) = utils::get_param(parameters, "playlist") {
        return Some(("playlist", id));
    }
    if let Some(id) = utils::get_param(parameters, "artist") {
        return Some(("artist", id));
    }
    if let Some(id) = utils::get_param(parameters, "episode") {
        return Some(("episode", id));
    }
    if let Some(id) = utils::get_param(parameters, "show") {
        return Some(("show", id));
    }
    None
}

fn build_embed_url(content_type: &str, id: &str, parameters: &Parameters) -> String {
    let mut params = Vec::new();

    if utils::get_param(parameters, "dark").is_some() {
        params.push("theme=0".to_string());
    }
    if utils::get_param(parameters, "compact").is_some() {
        params.push("view=coverart".to_string());
    }

    if params.is_empty() {
        format!("https://open.spotify.com/embed/{}/{}", content_type, id)
    } else {
        format!(
            "https://open.spotify.com/embed/{}/{}?{}",
            content_type,
            id,
            params.join("&")
        )
    }
}

pub fn render(
    data_start: Option<u32>,
    data_end: Option<u32>,
    parameters: &Parameters,
    ctx: &mut RenderContext,
) -> Markup {
    let (content_type, id) = match get_content_type_and_id(parameters) {
        Some(result) => result,
        None => {
            return html! {
                span class=(classes::ERROR) data-start=[data_start] data-end=[data_end] {
                    "Spotify: missing content parameter (track, album, playlist, artist, episode, or show)"
                }
            };
        }
    };

    let url = build_embed_url(content_type, &id, parameters);
    let lk = ctx.add_light_style(utils::build_style(parameters));
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        iframe
            class=(format!("{} {}", classes::EMBED, classes::EMBED_SPOTIFY))
            data-start=[data_start]
            data-end=[data_end]
            src=(url)
            data-lk=[lk]
            data-dk=[dk]
            frameborder="0"
            allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
            allowfullscreen
            loading="lazy"
        {}
    }
}
