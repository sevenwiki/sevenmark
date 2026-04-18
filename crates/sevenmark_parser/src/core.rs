use crate::context::ParseContext;
use crate::parser::document::document_parser;
use crate::parser::{InputSource, ParserInput};
use sevenmark_ast::{Element, ErrorElement, Span};
use winnow::stream::Location as StreamLocation;
use winnow::stream::Stream;

pub fn parse_document(input: &str) -> Vec<Element> {
    let context = ParseContext::new();

    let mut stateful_input = ParserInput {
        input: InputSource::new(input),
        state: context,
    };

    parse_document_input(&mut stateful_input)
}

pub(crate) fn parse_document_input(parser_input: &mut ParserInput) -> Vec<Element> {
    let initial_checkpoint = parser_input.checkpoint();
    let initial_state = parser_input.state.clone();

    match document_parser(parser_input) {
        Ok(mut elements) => {
            // Parse remaining content as Error element if any
            if !parser_input.input.is_empty() {
                let start = parser_input.current_token_start();
                let remaining = parser_input.input.peek_finish().to_string();
                parser_input.input.finish();
                let end = parser_input.previous_token_end();

                elements.push(Element::Error(ErrorElement {
                    span: Span { start, end },
                    value: remaining,
                }));
            }
            elements
        }
        Err(_) => {
            // If parser fails, treat entire input as single Error element
            parser_input.reset(&initial_checkpoint);
            parser_input.state = initial_state;

            let start = parser_input.current_token_start();
            let value = parser_input.input.peek_finish().to_string();
            parser_input.input.finish();
            let end = parser_input.previous_token_end();

            vec![Element::Error(ErrorElement {
                span: Span { start, end },
                value,
            })]
        }
    }
}
