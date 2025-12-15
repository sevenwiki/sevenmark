//! Media element renderer ([[...]])

use crate::context::RenderContext;
use crate::render_elements;
use crate::renderers::utils::{build_style_string, get_param_string};
use maud::{Markup, html};
use sevenmark_parser::ast::MediaElement;

/// Render media element (image, video, link)
pub fn render_media(elem: &MediaElement, ctx: &mut RenderContext) -> Markup {
    let style = build_style_string(&elem.parameters);
    let url = elem
        .resolved_info
        .as_ref()
        .map(|info| info.resolved_url.clone())
        .or_else(|| get_param_string(&elem.parameters, "url"))
        .unwrap_or_default();

    let alt = get_param_string(&elem.parameters, "alt").unwrap_or_default();
    let width = get_param_string(&elem.parameters, "width");
    let height = get_param_string(&elem.parameters, "height");

    // Determine media type from URL or parameters
    let media_type = get_param_string(&elem.parameters, "type")
        .unwrap_or_else(|| guess_media_type(&url).to_string());

    match media_type.as_str() {
        "video" => render_video(&url, width.as_deref(), height.as_deref(), style.as_deref()),
        "audio" => render_audio(&url),
        "link" => render_link(&url, &elem.content, &alt, ctx),
        _ => render_image(
            &url,
            &alt,
            width.as_deref(),
            height.as_deref(),
            style.as_deref(),
        ),
    }
}

fn guess_media_type(url: &str) -> &'static str {
    let lower = url.to_lowercase();
    if lower.ends_with(".mp4") || lower.ends_with(".webm") || lower.ends_with(".ogg") {
        "video"
    } else if lower.ends_with(".mp3") || lower.ends_with(".wav") || lower.ends_with(".flac") {
        "audio"
    } else {
        "image"
    }
}

fn render_image(
    url: &str,
    alt: &str,
    width: Option<&str>,
    height: Option<&str>,
    style: Option<&str>,
) -> Markup {
    html! {
        img class="sm-media sm-image"
            src=(url)
            alt=(alt)
            width=[width]
            height=[height]
            style=[style]
            loading="lazy";
    }
}

fn render_video(
    url: &str,
    width: Option<&str>,
    height: Option<&str>,
    style: Option<&str>,
) -> Markup {
    html! {
        video class="sm-media sm-video"
            src=(url)
            width=[width]
            height=[height]
            style=[style]
            controls {}
    }
}

fn render_audio(url: &str) -> Markup {
    html! {
        audio class="sm-media sm-audio" src=(url) controls {}
    }
}

fn render_link(
    url: &str,
    content: &[sevenmark_parser::ast::SevenMarkElement],
    alt: &str,
    ctx: &mut RenderContext,
) -> Markup {
    let link_text = if content.is_empty() {
        html! { (alt) }
    } else {
        render_elements(content, ctx)
    };

    html! {
        a class="sm-link" href=(url) {
            (link_text)
        }
    }
}
