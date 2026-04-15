//! Discord widget rendering (direct iframe embed)
//!
//! Embed URL: https://discord.com/widget?id={server_id}&theme={theme}
//!
//! Parameters:
//!   - id: Server ID (required)
//!   - dark: Dark theme (presence = enabled, default light)
//!   - width, height: Dimensions (overrides CSS default via data-lk)

use maud::{Markup, html};
use sevenmark_ast::Parameters;

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

fn build_embed_url(server_id: &str, parameters: &Parameters) -> String {
    let theme = if utils::get_param(parameters, "dark").is_some() {
        "dark"
    } else {
        "light"
    };

    format!(
        "https://discord.com/widget?id={}&theme={}",
        server_id, theme
    )
}

pub fn render(
    data_start: Option<u32>,
    data_end: Option<u32>,
    parameters: &Parameters,
    ctx: &mut RenderContext,
) -> Markup {
    let server_id = match utils::get_param(parameters, "id") {
        Some(id) => id,
        None => {
            return html! {
                span class=(classes::ERROR) data-start=[data_start] data-end=[data_end] {
                    "Discord: missing id parameter"
                }
            };
        }
    };

    let url = build_embed_url(&server_id, parameters);
    let lk = ctx.add_light_style(utils::build_style(parameters));
    let dk = ctx.add_dark_style(utils::build_dark_style(parameters));

    html! {
        iframe
            class=(format!("{} {}", classes::EMBED, classes::EMBED_DISCORD))
            data-start=[data_start]
            data-end=[data_end]
            src=(url)
            data-lk=[lk]
            data-dk=[dk]
            frameborder="0"
            allowtransparency="true"
            sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"
            loading="lazy"
        {}
    }
}
