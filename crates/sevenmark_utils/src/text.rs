use sevenmark_ast::Element;

/// Extract plain text content from a slice of Elements.
/// Only extracts Text and Escape values (shallow, non-recursive).
pub fn extract_plain_text(elements: &[Element]) -> String {
    elements
        .iter()
        .filter_map(|element| match element {
            Element::Text(text) => Some(text.value.as_str()),
            Element::Escape(escape) => Some(escape.value.as_str()),
            _ => None,
        })
        .collect::<String>()
}
