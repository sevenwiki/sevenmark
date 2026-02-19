//! Discord widget rendering (direct iframe embed)
//!
//! Embed URL: https://discord.com/widget?id={server_id}&theme={theme}
//!
//! Parameters:
//!   - id: Server ID (required)
//!   - dark: Dark theme (presence = enabled, default light)
//!   - width, height: Dimensions (default: 350x500)

use maud::{Markup, html};
use sevenmark_ast::Parameters;

use crate::classes;
use crate::render::utils::get_param;

fn build_embed_url(server_id: &str, parameters: &Parameters) -> String {
    let theme = if get_param(parameters, "dark").is_some() {
        "dark"
    } else {
        "light"
    };

    format!(
        "https://discord.com/widget?id={}&theme={}",
        server_id, theme
    )
}

pub fn render(data_start: Option<u32>, data_end: Option<u32>, parameters: &Parameters) -> Markup {
    let server_id = match get_param(parameters, "id") {
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
    let width = get_param(parameters, "width").unwrap_or_else(|| "350".to_string());
    let height = get_param(parameters, "height").unwrap_or_else(|| "500".to_string());

    html! {
        iframe
            class=(format!("{} {}", classes::EMBED, classes::EMBED_DISCORD))
            data-start=[data_start]
            data-end=[data_end]
            src=(url)
            width=(width)
            height=(height)
            frameborder="0"
            allowtransparency="true"
            sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"
            loading="lazy"
        {}
    }
}
