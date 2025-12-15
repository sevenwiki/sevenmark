//! Common style utilities for rendering
//!
//! Defines parameter → CSS mapping rules for SevenMark elements.

use sevenmark_parser::ast::{Parameters, SevenMarkElement};

/// Parameter to CSS property mapping
///
/// | Parameter | CSS Property |
/// |-----------|--------------|
/// | block     | display: block |
/// | inline    | display: inline |
/// | style     | (raw CSS)    |
/// | color     | color        |
/// | bgcolor / bg | background-color |
/// | size      | font-size    |
/// | width     | width        |
/// | height    | height       |
/// | align     | text-align   |
/// | opacity   | opacity      |
/// | border    | border       |
/// | padding   | padding      |
/// | margin    | margin       |
/// | radius    | border-radius |

/// Extract string value from parameter (concatenate Text elements)
pub fn get_param_string(params: &Parameters, key: &str) -> Option<String> {
    params.get(key).map(|p| extract_text_content(&p.value))
}

/// Extract text content from elements (simple concatenation)
fn extract_text_content(elements: &[SevenMarkElement]) -> String {
    let mut result = String::new();
    for elem in elements {
        match elem {
            SevenMarkElement::Text(t) => result.push_str(&t.content),
            SevenMarkElement::Escape(e) => result.push_str(&e.content),
            _ => {} // Ignore non-text elements in parameters
        }
    }
    result
}

/// Build CSS style string from parameters
pub fn build_style_string(params: &Parameters) -> Option<String> {
    let mut styles = Vec::new();

    // Display mode (first, so it can be overridden by more specific styles)
    // #block → display: block, #inline → display: inline
    if params.contains_key("block") {
        styles.push("display: block".to_string());
    } else if params.contains_key("inline") {
        styles.push("display: inline".to_string());
    }

    // Direct style attribute (raw CSS) - trim trailing semicolons
    if let Some(style) = get_param_string(params, "style") {
        let trimmed = style.trim_end_matches(';').trim();
        if !trimmed.is_empty() {
            styles.push(trimmed.to_string());
        }
    }

    // Text color
    if let Some(color) = get_param_string(params, "color") {
        styles.push(format!("color: {}", color));
    }

    // Background color
    if let Some(bg) = get_param_string(params, "bgcolor").or_else(|| get_param_string(params, "bg"))
    {
        styles.push(format!("background-color: {}", bg));
    }

    // Font size
    if let Some(size) = get_param_string(params, "size") {
        styles.push(format!("font-size: {}", size));
    }

    // Dimensions
    if let Some(width) = get_param_string(params, "width") {
        styles.push(format!("width: {}", width));
    }
    if let Some(height) = get_param_string(params, "height") {
        styles.push(format!("height: {}", height));
    }

    // Text alignment
    if let Some(align) = get_param_string(params, "align") {
        styles.push(format!("text-align: {}", align));
    }

    // Opacity
    if let Some(opacity) = get_param_string(params, "opacity") {
        styles.push(format!("opacity: {}", opacity));
    }

    // Border
    if let Some(border) = get_param_string(params, "border") {
        styles.push(format!("border: {}", border));
    }

    // Spacing
    if let Some(padding) = get_param_string(params, "padding") {
        styles.push(format!("padding: {}", padding));
    }
    if let Some(margin) = get_param_string(params, "margin") {
        styles.push(format!("margin: {}", margin));
    }

    // Border radius
    if let Some(radius) = get_param_string(params, "radius") {
        styles.push(format!("border-radius: {}", radius));
    }

    if styles.is_empty() {
        None
    } else {
        Some(styles.join("; "))
    }
}

/// Get class attribute from parameters
pub fn get_class(params: &Parameters) -> Option<String> {
    get_param_string(params, "class")
}

/// Get id attribute from parameters
pub fn get_id(params: &Parameters) -> Option<String> {
    get_param_string(params, "id")
}
