use std::collections::HashSet;

use sevenmark_ast::Element;
use sevenmark_utils::extract_plain_text;
use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;

/// Collects LSP diagnostics from parsed AST.
///
/// Two kinds of diagnostics:
/// - **Error**: `ErrorElement` nodes (parser failures)
/// - **Warning**: `VariableElement` referencing an undefined variable
pub fn collect_diagnostics(state: &DocumentState) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut defined_vars = HashSet::new();

    // Pass 1: collect error diagnostics and defined variable names
    visit_elements(&state.elements, &mut |element| {
        match element {
            Element::Error(e) => {
                let (start, end) = state.line_index.span_to_range(&state.text, &e.span);
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(start.0, start.1),
                        Position::new(end.0, end.1),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("sevenmark".to_string()),
                    message: format!("Parse error: {}", truncate(&e.value, 80)),
                    ..Default::default()
                });
            }
            Element::Define(e) => {
                if let Some(name_param) = e.parameters.get("name") {
                    let name = extract_plain_text(&name_param.value);
                    if !name.is_empty() {
                        defined_vars.insert(name);
                    }
                }
            }
            _ => {}
        }
    });

    // Pass 2: emit warnings for undefined variable references
    visit_elements(&state.elements, &mut |element| {
        if let Element::Variable(v) = element
            && !defined_vars.contains(&v.name)
        {
            let (start, end) = state.line_index.span_to_range(&state.text, &v.span);
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(start.0, start.1),
                    Position::new(end.0, end.1),
                ),
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("sevenmark".to_string()),
                message: format!("Undefined variable: {}", v.name),
                ..Default::default()
            });
        }
    });

    diagnostics
}

/// Truncates a string for diagnostic messages.
fn truncate(s: &str, max_len: usize) -> String {
    let clean = s.replace('\n', "\\n");
    if clean.chars().count() <= max_len {
        clean
    } else {
        let truncated: String = clean.chars().take(max_len).collect();
        truncated + "…"
    }
}