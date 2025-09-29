use crate::sevenmark::ast::{ErrorElement, Location, SevenMarkElement};
use crate::sevenmark::context::ParseContext;
use crate::sevenmark::parser::document::document_parser;
use crate::sevenmark::visitor::preprocessor::{PreVisitor, PreprocessInfo, SevenMarkPreprocessor};
use crate::sevenmark::{InputSource, ParserInput};
use line_span::LineSpanExt;
use std::collections::HashSet;
use winnow::stream::Location as StreamLocation;

pub fn parse_document(input: &str) -> Vec<SevenMarkElement> {
    // Pre-calculate all line start positions for O(1) lookups
    let line_starts: HashSet<usize> = input.line_spans().map(|span| span.range().start).collect();

    let mut context = ParseContext::new();
    context.line_starts = line_starts;

    let mut stateful_input = ParserInput {
        input: InputSource::new(&input),
        state: context,
    };

    match document_parser(&mut stateful_input) {
        Ok(mut elements) => {
            // Parse remaining content as Error element if any
            if !stateful_input.input.is_empty() {
                let start = stateful_input.input.current_token_start();
                let remaining = stateful_input.input.to_string();
                let end = start + remaining.len();

                elements.push(SevenMarkElement::Error(ErrorElement {
                    location: Location { start, end },
                    content: remaining,
                }));
            }
            elements
        }
        Err(_) => {
            // If parser fails, treat entire input as single Error element
            vec![SevenMarkElement::Error(ErrorElement {
                location: Location {
                    start: 0,
                    end: input.len(),
                },
                content: input.to_string(),
            })]
        }
    }
}

pub fn parse_document_with_preprocessing(input: &str) -> (Vec<SevenMarkElement>, PreprocessInfo) {
    // Step 1: Basic parsing
    let mut elements = parse_document(input);

    // Step 2: Variable resolution & preprocessing info collection
    let preprocess_info = SevenMarkPreprocessor::preprocess(&mut elements);

    (elements, preprocess_info)
}
