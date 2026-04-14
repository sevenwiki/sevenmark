//! Common rendering utilities

use std::hash::{Hash, Hasher};

use sevenmark_ast::{Element, Parameters};

/// Extract plain text from elements, recursing into nested children.
pub fn extract_text(elements: &[Element]) -> String {
    use sevenmark_ast::Traversable;

    fn collect(el: &Element, out: &mut String) {
        match el {
            Element::Text(text) => out.push_str(&text.value),
            Element::Escape(escape) => out.push_str(&escape.value),
            other => other.traverse_children_ref(&mut |child| collect(child, out)),
        }
    }

    let mut result = String::new();
    for el in elements {
        collect(el, &mut result);
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

/// Build a shared CSS rule for a sanitized style payload.
///
/// `selector_prefix` is prepended before the attribute selector, e.g. `""` for
/// light mode (`[data-lk="..."]`) or `".dark "` for dark mode
/// (`.dark [data-dk="..."]`).  Returns `(hash, css_rule)`.
fn build_style_rule(css: &str, attr: &str, selector_prefix: &str) -> (String, String) {
    let hash = style_hash(css);
    let escaped = super::sanitize::escape_style_close_tag(css);
    let rule = format!("{selector_prefix}[{attr}=\"{hash}\"]{{{escaped}}}");
    (hash, rule)
}

/// Build the shared light-mode CSS rule. Returns `(data_lk_hash, css_rule)`.
pub fn build_light_style_rule(css: &str) -> (String, String) {
    build_style_rule(css, "data-lk", "")
}

/// Build the shared dark-mode CSS rule. Returns `(data_dk_hash, css_rule)`.
pub fn build_dark_style_rule(css: &str) -> (String, String) {
    build_style_rule(css, "data-dk", ".dark ")
}

fn style_hash(css: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    css.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Build a sanitized dark-style declaration list from common style parameters.
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

/// Build a sanitized light-style declaration list from common style parameters.
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
