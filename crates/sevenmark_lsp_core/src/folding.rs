use ls_types::{FoldingRange, FoldingRangeKind};
use sevenmark_ast::{Element, Traversable};

use crate::document::DocumentState;

/// Collects folding ranges from multi-line block elements in the AST.
pub fn collect_folding_ranges(state: &DocumentState) -> Vec<FoldingRange> {
    let mut ranges = Vec::new();
    visit_for_folding(&state.elements, state, &mut ranges);
    ranges
}

fn visit_for_folding(elements: &[Element], state: &DocumentState, ranges: &mut Vec<FoldingRange>) {
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

        let (start_line, _) = state
            .line_index
            .byte_offset_to_position(&state.text, span.start);
        let (end_line, _) = state
            .line_index
            .byte_offset_to_position(&state.text, span.end);

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

#[cfg(test)]
mod tests {
    use super::*;
    use ls_types::FoldingRangeKind;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    #[test]
    fn multiline_code_block_produces_folding_range() {
        let text = "{{{#code\nline1\nline2\n}}}";
        let state = make_state(text);
        let ranges = collect_folding_ranges(&state);
        assert!(!ranges.is_empty(), "expected at least one folding range");
        let code_range = &ranges[0];
        assert_eq!(code_range.kind, Some(FoldingRangeKind::Region));
        assert!(code_range.end_line > code_range.start_line);
    }

    #[test]
    fn single_line_block_no_folding_range() {
        let text = "{{{#code}}}";
        let state = make_state(text);
        let ranges = collect_folding_ranges(&state);
        // Single-line elements should not produce folding ranges
        let region_ranges: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Region))
            .collect();
        assert!(
            region_ranges.is_empty(),
            "expected no folding ranges for single-line block"
        );
    }

    #[test]
    fn multiline_comment_produces_comment_folding() {
        let text = "text /* comment\nline 2\nline 3 */ after";
        let state = make_state(text);
        let ranges = collect_folding_ranges(&state);
        let comment_ranges: Vec<_> = ranges
            .iter()
            .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
            .collect();
        assert!(!comment_ranges.is_empty(), "expected comment folding range");
        assert!(comment_ranges[0].end_line > comment_ranges[0].start_line);
    }
}
