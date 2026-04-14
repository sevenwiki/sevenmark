//! Common rendering utilities

use std::hash::{Hash, Hasher};

use maud::{Markup, PreEscaped, html};
use sevenmark_ast::{Element, Parameters};

/// Extract plain text from elements
pub fn extract_text(elements: &[Element]) -> String {
    let mut result = String::new();
    for el in elements {
        match el {
            Element::Text(text) => result.push_str(&text.value),
            Element::Escape(escape) => result.push_str(&escape.value),
            _ => {}
        }
    }
    result
}

/// Get parameter value as text
pub fn get_param(params: &Parameters, key: &str) -> Option<String> {
    params.get(key).map(|p| extract_text(&p.value))
}

/// Keep the base renderer class and append optional user-defined `#class`.
pub fn merge_class(base: &str, params: &Parameters) -> String {
    match get_param(params, "class")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        Some(extra) => format!("{base} {extra}"),
        None => base.to_string(),
    }
}

pub fn param_class(params: &Parameters) -> Option<String> {
    get_param(params, "class")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Build a `<style>.dark [data-dk="hash"] { … }</style>` tag for the given dark style.
///
/// The `data-dk` value is a hash of the CSS text, so identical dark styles share
/// the same selector.  Every declaration is strengthened with `!important` so it
/// overrides any inline `style` attribute on the same element.
/// Returns `(None, empty markup)` when there is no dark style.
pub fn dark_style_parts(dark_style: Option<String>) -> (Option<String>, Markup) {
    match dark_style {
        Some(ds) => {
            let dk = dark_style_hash(&ds);
            let important = add_important(&ds);
            let escaped = super::sanitize::escape_style_close_tag(&important);
            let rule = format!(".dark [data-dk=\"{dk}\"]{{{escaped}}}");
            let tag = html! { style { (PreEscaped(rule)) } };
            (Some(dk), tag)
        }
        None => (None, html! {}),
    }
}

fn dark_style_hash(css: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    css.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Append `!important` to every declaration in a sanitized inline-style string.
fn add_important(css: &str) -> String {
    css.split(';')
        .map(str::trim)
        .filter(|d| !d.is_empty())
        .map(|d| {
            if d.ends_with("!important") {
                d.to_string()
            } else {
                format!("{d} !important")
            }
        })
        .collect::<Vec<_>>()
        .join(";")
}

pub fn build_dark_style(params: &Parameters) -> Option<String> {
    let mut styles = Vec::new();

    // Explicit raw dark style
    if let Some(style) = get_param(params, "dark-style") {
        styles.push(style);
    }
    if let Some(size) = get_param(params, "dark-size") {
        styles.push(format!("font-size:{}", size));
    }
    if let Some(color) = get_param(params, "dark-color") {
        styles.push(format!("color:{}", color));
    }
    if let Some(bg) = get_param(params, "dark-bgcolor") {
        styles.push(format!("background-color:{}", bg));
    }
    if let Some(opacity) = get_param(params, "dark-opacity") {
        styles.push(format!("opacity:{}", opacity));
    }

    if styles.is_empty() {
        None
    } else {
        let raw = styles.join(";");
        let sanitized = super::sanitize::sanitize_inline_style(&raw);
        if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        }
    }
}

/// Build inline style string from common style parameters
pub fn build_style(params: &Parameters) -> Option<String> {
    let mut styles = Vec::new();

    if let Some(style) = get_param(params, "style") {
        styles.push(style);
    }
    if let Some(size) = get_param(params, "size") {
        styles.push(format!("font-size:{}", size));
    }
    if let Some(color) = get_param(params, "color") {
        styles.push(format!("color:{}", color));
    }
    if let Some(bg) = get_param(params, "bgcolor") {
        styles.push(format!("background-color:{}", bg));
    }
    if let Some(opacity) = get_param(params, "opacity") {
        styles.push(format!("opacity:{}", opacity));
    }

    if styles.is_empty() {
        None
    } else {
        let raw = styles.join(";");
        let sanitized = super::sanitize::sanitize_inline_style(&raw);
        if sanitized.is_empty() {
            None
        } else {
            Some(sanitized)
        }
    }
}
