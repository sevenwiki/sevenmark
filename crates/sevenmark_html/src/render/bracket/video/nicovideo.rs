//! NicoNico (nicovideo) video rendering (direct iframe embed)
//!
//! Embed URL: https://embed.nicovideo.jp/watch/{id}
//!
//! Parameters:
//!   - id: Video ID (required, e.g., "sm9", "so39402840")
//!   - width, height: Dimensions (overrides CSS default via data-lk)
//!   - from: Start time in seconds
//!   - autoplay: Auto-play (present = enabled)

use maud::{Markup, html};
use sevenmark_ast::Parameters;

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

fn build_embed_url(id: &str, parameters: &Parameters) -> String {
    let mut params = Vec::new();

    if let Some(from) = utils::get_param(parameters, "from") {
        params.push(format!("from={}", from));
    }
    if utils::get_param(parameters, "autoplay").is_some() {
        params.push("autoplay=1".to_string());
    }

    if params.is_empty() {
        format!("https://embed.nicovideo.jp/watch/{}", id)
    } else {
        format!(
            "https://embed.nicovideo.jp/watch/{}?{}",
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
    let id = match utils::get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) data-start=[data_start] data-end=[data_end] {
                    "NicoNico: missing id parameter"
                }
            };
        }
    };

    let url = build_embed_url(&id, parameters);
    let lk = ctx.add_light_style(utils::build_style(parameters));
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        iframe
            class=(format!("{} {}", classes::EMBED, classes::EMBED_NICOVIDEO))
            data-start=[data_start]
            data-end=[data_end]
            src=(url)
            data-lk=[lk]
            data-dk=[dk]
            frameborder="0"
            allow="autoplay"
            allowfullscreen
            loading="lazy"
        {}
    }
}
