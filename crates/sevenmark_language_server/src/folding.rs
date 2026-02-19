use sevenmark_ast::{Element, Traversable};
use tower_lsp_server::ls_types::{FoldingRange, FoldingRangeKind};

use crate::document::DocumentState;

/// Collects folding ranges from multi-line block elements in the AST.
pub fn collect_folding_ranges(state: &DocumentState) -> Vec<FoldingRange> {
    let mut ranges = Vec::new();
    visit_for_folding(&state.elements, state, &mut ranges);
    ranges
}

fn visit_for_folding(
    elements: &[Element],
    state: &DocumentState,
    ranges: &mut Vec<FoldingRange>,
) {
    for element in elements {
        let (span, kind) = match element {
            Element::Fold(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Code(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Table(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::List(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::BlockQuote(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::If(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Literal(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Include(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Styled(e) => (&e.span, Some(FoldingRangeKind::Region)),
            Element::Comment(e) => (&e.span, Some(FoldingRangeKind::Comment)),
            _ => {
                // Recurse into non-block container elements
                element.traverse_children_ref(&mut |child| {
                    visit_for_folding(std::slice::from_ref(child), state, ranges);
                });
                continue;
            }
        };

        let (start_line, _) = state.line_index.byte_offset_to_position(&state.text, span.start);
        let (end_line, _) = state.line_index.byte_offset_to_position(&state.text, span.end);

        // Only emit folding range if the element spans multiple lines
        if end_line > start_line {
            ranges.push(FoldingRange {
                start_line,
                start_character: None,
                end_line,
                end_character: None,
                kind,
                collapsed_text: None,
            });
        }

        // Recurse into children of block elements too
        element.traverse_children_ref(&mut |child| {
            visit_for_folding(std::slice::from_ref(child), state, ranges);
        });
    }
}