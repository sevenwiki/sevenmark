use sevenmark_ast::Element;
use sevenmark_utils::extract_plain_text;

pub(crate) fn normalized_plain_text(elements: &[Element]) -> Option<String> {
    let raw = extract_plain_text(elements);
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
