use sevenmark_ast::Element;
use ls_types::{Location, Position, Range, Uri};

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;

/// Finds the definition site for the element at the given byte offset.
///
/// Currently supports:
/// - `[var(name)]` → jumps to the first `{{{#define #name="name" ...}}}` in the document
pub fn find_definition(state: &DocumentState, uri: &Uri, byte_offset: usize) -> Option<Location> {
    let var_name = find_variable_at(state, byte_offset)?;
    find_define_location(state, uri, &var_name)
}

/// Returns the variable name if a `VariableElement` spans the given byte offset.
fn find_variable_at(state: &DocumentState, byte_offset: usize) -> Option<String> {
    let mut result = None;
    visit_elements(&state.elements, &mut |element| {
        if result.is_some() {
            return;
        }
        if let Element::Variable(v) = element {
            if v.span.start <= byte_offset && byte_offset < v.span.end {
                result = Some(v.name.clone());
            }
        }
    });
    result
}

/// Finds the first `{{{#define #name="target" ...}}}` and returns its location.
fn find_define_location(state: &DocumentState, uri: &Uri, target: &str) -> Option<Location> {
    let mut result = None;
    visit_elements(&state.elements, &mut |element| {
        if result.is_some() {
            return;
        }
        if let Element::Define(d) = element {
            if d.parameters.contains_key(target) {
                let (start, end) = state.line_index.span_to_range(&state.text, &d.span);
                result = Some(Location {
                    uri: uri.clone(),
                    range: Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1)),
                });
            }
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    fn test_uri() -> Uri {
        "file:///test.sm".parse().unwrap()
    }

    #[test]
    fn var_with_define_returns_location() {
        let text = "{{{#define #x=\"v\"}}}[var(x)]";
        let state = make_state(text);
        let uri = test_uri();
        // Byte offset inside [var(x)]
        let var_start = text.find("[var(x)]").unwrap();
        let byte_offset = var_start + 5; // points to 'x'
        let loc = find_definition(&state, &uri, byte_offset);
        assert!(loc.is_some(), "expected definition location");
        assert_eq!(loc.unwrap().uri, uri);
    }

    #[test]
    fn var_without_define_returns_none() {
        let text = "[var(x)]";
        let state = make_state(text);
        let uri = test_uri();
        let byte_offset = 5; // inside [var(x)]
        let loc = find_definition(&state, &uri, byte_offset);
        assert!(loc.is_none());
    }

    #[test]
    fn cursor_not_on_variable_returns_none() {
        let text = "hello world";
        let state = make_state(text);
        let uri = test_uri();
        let loc = find_definition(&state, &uri, 3);
        assert!(loc.is_none());
    }
}
