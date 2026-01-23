use crate::ast::{Element, NullElement, Span};
use crate::parser::ParserInput;
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse null macro [null] -> returns Null element
pub fn macro_null_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.input.current_token_start();
    literal("[null]").parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(Element::Null(NullElement {
        span: Span { start, end },
    }))
}
