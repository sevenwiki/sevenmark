//! Media element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::MediaElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &MediaElement, ctx: &mut RenderContext) -> Markup {
    let url = e
        .resolved_info
        .as_ref()
        .map(|r| r.resolved_url.as_str())
        .unwrap_or("");

    let is_file = e.parameters.contains_key("file");
    let style = utils::build_style(&e.parameters);

    let caption = if e.content.is_empty() {
        None
    } else {
        Some(render_elements(&e.content, ctx))
    };

    if is_file {
        html! {
            figure class=(classes::MEDIA_IMAGE) style=[style] {
                img src=(url) alt="" loading="lazy";
                @if let Some(cap) = caption {
                    figcaption { (cap) }
                }
            }
        }
    } else {
        html! {
            a class=(classes::MEDIA_LINK) href=(url) style=[style] {
                @if let Some(cap) = caption {
                    (cap)
                } @else {
                    (url)
                }
            }
        }
    }
}
