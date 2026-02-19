//! Media element rendering

use maud::{Markup, html};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use sevenmark_ast::{Element, Parameters, ResolvedMediaInfo, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

fn sanitize_external_url(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        Some(trimmed.to_string())
    } else {
        None
    }
}

// Encode path segment with RFC 3986 unreserved characters left as-is.
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

fn build_internal_href(base: &str, title: &str) -> String {
    let encoded = utf8_percent_encode(title, PATH_SEGMENT_ENCODE_SET).to_string();
    format!("{}{}", base, encoded)
}

pub fn render(
    span: &Span,
    parameters: &Parameters,
    children: &[Element],
    resolved_info: Option<&ResolvedMediaInfo>,
    ctx: &mut RenderContext,
) -> Markup {
    let data_start = ctx.span_start(span);
    let data_end = ctx.span_end(span);
    let style = utils::build_style(parameters);

    // 파일 정보 추출 (URL, 유효성, 크기)
    let file_info = resolved_info.and_then(|r| r.file.as_ref());
    let image_src: Option<String> = file_info.map(|f| {
        if let Some(base) = ctx.config.file_base_url {
            format!("{}{}", base, f.url)
        } else {
            f.url.clone()
        }
    });
    let image_valid = file_info.is_some_and(|f| f.is_valid);
    let image_width = file_info.and_then(|f| f.width);
    let image_height = file_info.and_then(|f| f.height);

    // alt 텍스트: 파일 제목 사용
    let alt_text = utils::get_param(parameters, "file").unwrap_or_default();

    // href 우선순위: url > document > category > user
    let href: Option<String> = resolved_info.and_then(|r| {
        if let Some(url) = r.url.as_deref() {
            return sanitize_external_url(url);
        }

        r.document
            .as_ref()
            .and_then(|d| {
                ctx.config
                    .document_base_url
                    .map(|base| build_internal_href(base, &d.title))
            })
            .or_else(|| {
                r.category.as_ref().and_then(|c| {
                    ctx.config
                        .category_base_url
                        .map(|base| build_internal_href(base, &c.title))
                })
            })
            .or_else(|| {
                r.user.as_ref().and_then(|u| {
                    ctx.config
                        .user_base_url
                        .map(|base| build_internal_href(base, &u.title))
                })
            })
    });

    // 링크 유효성 (외부 url은 항상 valid 취급)
    let href_valid = resolved_info
        .map(|r| {
            if let Some(url) = r.url.as_deref() {
                sanitize_external_url(url).is_some()
            } else if let Some(doc) = &r.document {
                doc.is_valid
            } else if let Some(cat) = &r.category {
                cat.is_valid
            } else if let Some(user) = &r.user {
                user.is_valid
            } else {
                true
            }
        })
        .unwrap_or(true);

    let caption = if children.is_empty() {
        None
    } else {
        Some(render_elements(children, ctx))
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
                a
                    class=(link_class)
                    data-start=[data_start]
                    data-end=[data_end]
                    href=(link)
                    style=[style]
                {
                    figure class=(classes::MEDIA_IMAGE) {
                        @if image_valid {
                            img src=(src) width=[image_width] height=[image_height] alt=(alt_text) loading="lazy";
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
                figure
                    class=(classes::MEDIA_IMAGE)
                    data-start=[data_start]
                    data-end=[data_end]
                    style=[style]
                {
                    @if image_valid {
                        img src=(src) width=[image_width] height=[image_height] alt=(alt_text) loading="lazy";
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
            a
                class=(link_class)
                data-start=[data_start]
                data-end=[data_end]
                href=(link)
                style=[style]
            {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_external_url_allows_http_https() {
        assert_eq!(
            sanitize_external_url("https://example.com/a?b=1"),
            Some("https://example.com/a?b=1".to_string())
        );
        assert_eq!(
            sanitize_external_url("HTTP://example.com"),
            Some("HTTP://example.com".to_string())
        );
    }

    #[test]
    fn sanitize_external_url_rejects_unsafe_schemes() {
        assert_eq!(sanitize_external_url("javascript:alert(1)"), None);
        assert_eq!(sanitize_external_url("data:text/html,hi"), None);
        assert_eq!(sanitize_external_url(""), None);
    }

    #[test]
    fn build_internal_href_percent_encodes_title() {
        let href = build_internal_href("/Document/", "A B/#?");
        assert_eq!(href, "/Document/A%20B%2F%23%3F");
    }
}
