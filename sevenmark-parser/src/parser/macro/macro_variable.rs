use crate::ast::{Location, SevenMarkElement, VariableElement};
use crate::parser::ParserInput;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_until};

pub fn macro_variable_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let content = delimited(literal("[var("), take_until(0.., ")]"), literal(")]"))
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Variable(VariableElement {
        location: Location { start, end },
        content: content.to_string(),
    }))
}
