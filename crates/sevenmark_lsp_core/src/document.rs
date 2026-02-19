use sevenmark_ast::Element;
use sevenmark_parser::core::parse_document;
use sevenmark_utils::LineIndex;

/// Per-document state cached between edits.
///
/// Rebuilt on every `did_change` / `did_open` notification.
/// Keeps the parsed AST and a precomputed line index for
/// fast byte-offset → LSP position conversion.
pub struct DocumentState {
    pub text: String,
    pub elements: Vec<Element>,
    pub line_index: LineIndex,
}

impl DocumentState {
    /// Parses the text and builds all derived indices.
    pub fn new(text: String) -> Self {
        let elements = parse_document(&text);
        let line_index = LineIndex::new(&text);
        Self {
            text,
            elements,
            line_index,
        }
    }
}