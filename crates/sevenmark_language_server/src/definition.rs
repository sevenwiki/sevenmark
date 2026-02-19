use sevenmark_ast::Element;
use sevenmark_utils::extract_plain_text;
use tower_lsp_server::ls_types::{Location, Position, Range, Uri};

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
            if let Some(name_param) = d.parameters.get("name") {
                if extract_plain_text(&name_param.value) == target {
                    let (start, end) = state.line_index.span_to_range(&state.text, &d.span);
                    result = Some(Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(start.0, start.1),
                            Position::new(end.0, end.1),
                        ),
                    });
                }
            }
        }
    });
    result
}
