use std::collections::HashSet;

use ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use sevenmark_ast::Element;

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
    visit_elements(&state.elements, &mut |element| match element {
        Element::Error(e) => {
            let (start, end) = state.line_index.span_to_range(&state.text, &e.span);
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1)),
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("sevenmark".to_string()),
                message: format!("Parse error: {}", truncate(&e.value, 80)),
                ..Default::default()
            });
        }
        Element::Define(e) => {
            for name in e.parameters.keys() {
                defined_vars.insert(name.clone());
            }
        }
        _ => {}
    });

    // Pass 2: emit warnings for undefined variable references
    visit_elements(&state.elements, &mut |element| {
        if let Element::Variable(v) = element
            && !defined_vars.contains(&v.name)
        {
            let (start, end) = state.line_index.span_to_range(&state.text, &v.span);
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use ls_types::DiagnosticSeverity;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    #[test]
    fn clean_document_no_diagnostics() {
        let state = make_state("hello world");
        let diags = collect_diagnostics(&state);
        assert!(diags.is_empty());
    }

    #[test]
    fn undefined_variable_warning() {
        let state = make_state("[var(x)]");
        let diags = collect_diagnostics(&state);
        let warning = diags
            .iter()
            .find(|d| d.severity == Some(DiagnosticSeverity::WARNING));
        assert!(warning.is_some(), "expected a WARNING diagnostic");
        let warning = warning.unwrap();
        assert!(warning.message.contains("Undefined variable"));
        assert!(warning.message.contains("x"));
    }

    #[test]
    fn defined_variable_no_warning() {
        let state = make_state("{{{#define #x=\"v\"}}}[var(x)]");
        let diags = collect_diagnostics(&state);
        let warnings: Vec<_> = diags
            .iter()
            .filter(|d| d.severity == Some(DiagnosticSeverity::WARNING))
            .collect();
        assert!(
            warnings.is_empty(),
            "expected no warnings but got: {warnings:?}"
        );
    }

    #[test]
    fn parser_error_diagnostic() {
        // Intentionally malformed input — unclosed brace block
        let state = make_state("{{{");
        let diags = collect_diagnostics(&state);
        let errors: Vec<_> = diags
            .iter()
            .filter(|d| d.severity == Some(DiagnosticSeverity::ERROR))
            .collect();
        assert!(!errors.is_empty(), "expected at least one ERROR diagnostic");
    }
}
