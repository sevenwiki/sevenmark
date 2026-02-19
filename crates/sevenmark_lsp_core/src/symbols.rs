use ls_types::{DocumentSymbol, Position, Range, SymbolKind};
use sevenmark_ast::{Element, Traversable};
use sevenmark_utils::extract_plain_text;

use crate::document::DocumentState;

/// Extracts document symbols (headers and variable definitions) from the AST.
pub fn collect_document_symbols(state: &DocumentState) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    visit_for_symbols(&state.elements, state, &mut symbols);
    symbols
}

/// Recursively walks elements collecting symbols.
fn visit_for_symbols(
    elements: &[Element],
    state: &DocumentState,
    symbols: &mut Vec<DocumentSymbol>,
) {
    for element in elements {
        match element {
            Element::Header(h) => {
                let name = extract_plain_text(&h.children);
                let name = if name.is_empty() {
                    format!("Header (level {})", h.level)
                } else {
                    name
                };
                let (start, end) = state.line_index.span_to_range(&state.text, &h.span);
                let range =
                    Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1));

                #[allow(deprecated)]
                symbols.push(DocumentSymbol {
                    name,
                    detail: Some(format!("Level {}", h.level)),
                    kind: SymbolKind::STRING,
                    range,
                    selection_range: range,
                    children: None,
                    tags: None,
                    deprecated: None,
                });

                visit_for_symbols(&h.children, state, symbols);
            }
            Element::Define(d) => {
                let (start, end) = state.line_index.span_to_range(&state.text, &d.span);
                let range =
                    Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1));

                for var_name in d.parameters.keys() {
                    #[allow(deprecated)]
                    symbols.push(DocumentSymbol {
                        name: var_name.clone(),
                        detail: Some("Define".to_string()),
                        kind: SymbolKind::VARIABLE,
                        range,
                        selection_range: range,
                        children: None,
                        tags: None,
                        deprecated: None,
                    });
                }
            }
            other => {
                other.traverse_children_ref(&mut |child| {
                    visit_for_symbols(std::slice::from_ref(child), state, symbols);
                });
            }
        }
    }
}
