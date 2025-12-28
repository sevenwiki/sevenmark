//! Media element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::MediaElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &MediaElement, ctx: &mut RenderContext) -> Markup {
    let resolved = e.resolved_info.as_ref();
    let style = utils::build_style(&e.parameters);

    // 이미지 URL (file이 있으면)
    let image_src = resolved.and_then(|r| r.file.as_ref()).map(|f| f.url.as_str());
    let image_valid = resolved
        .and_then(|r| r.file.as_ref())
        .map(|f| f.is_valid)
        .unwrap_or(false);

    // href 우선순위: url > document > category
    let href: Option<String> = resolved.and_then(|r| {
        r.url.clone()
            .or_else(|| {
                r.document.as_ref().and_then(|d| {
                    ctx.config
                        .document_base_url
                        .map(|base| format!("{}{}", base, d.title))
                })
            })
            .or_else(|| {
                r.category.as_ref().and_then(|c| {
                    ctx.config
                        .category_base_url
                        .map(|base| format!("{}{}", base, c.title))
                })
            })
    });

    // 링크 유효성 (외부 url은 항상 valid 취급)
    let href_valid = resolved
        .map(|r| {
            if r.url.is_some() {
                true // 외부 링크는 항상 valid
            } else if let Some(doc) = &r.document {
                doc.is_valid
            } else if let Some(cat) = &r.category {
                cat.is_valid
            } else {
                true
            }
        })
        .unwrap_or(true);

    let caption = if e.content.is_empty() {
        None
    } else {
        Some(render_elements(&e.content, ctx))
    };

    // 이미지가 있는 경우
    if let Some(src) = image_src {
        if let Some(ref link) = href {
            // 이미지 + 링크
            let link_class = if href_valid {
                classes::MEDIA_LINK
            } else {
                classes::MEDIA_LINK_INVALID
            };
            html! {
                a class=(link_class) href=(link) style=[style] {
                    figure class=(classes::MEDIA_IMAGE) {
                        @if image_valid {
                            img src=(src) alt="" loading="lazy";
                        } @else {
                            span class=(classes::MEDIA_IMAGE_BROKEN) {}
                        }
                        @if let Some(cap) = caption {
                            figcaption { (cap) }
                        }
                    }
                }
            }
        } else {
            // 이미지만
            html! {
                figure class=(classes::MEDIA_IMAGE) style=[style] {
                    @if image_valid {
                        img src=(src) alt="" loading="lazy";
                    } @else {
                        span class=(classes::MEDIA_IMAGE_BROKEN) {}
                    }
                    @if let Some(cap) = caption {
                        figcaption { (cap) }
                    }
                }
            }
        }
    } else if let Some(ref link) = href {
        // 링크만 (이미지 없음)
        let link_class = if href_valid {
            classes::MEDIA_LINK
        } else {
            classes::MEDIA_LINK_INVALID
        };
        html! {
            a class=(link_class) href=(link) style=[style] {
                @if let Some(cap) = caption {
                    (cap)
                } @else {
                    (link)
                }
            }
        }
    } else {
        // 아무것도 없음
        html! {}
    }
}
