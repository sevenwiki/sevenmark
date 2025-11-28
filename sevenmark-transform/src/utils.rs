use sevenmark_parser::ast::SevenMarkElement;

/// Extract plain text content from a slice of SevenMarkElements
pub fn extract_plain_text(elements: &[SevenMarkElement]) -> String {
    elements
        .iter()
        .filter_map(|element| match element {
            SevenMarkElement::Text(text_element) => Some(text_element.content.as_str()),
            SevenMarkElement::Escape(escape_element) => Some(escape_element.content.as_str()),
            _ => None,
        })
        .collect::<String>()
}
