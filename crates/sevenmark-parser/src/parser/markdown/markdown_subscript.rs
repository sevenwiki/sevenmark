use super::super::element::element_parser;
use super::super::utils::with_depth;
use crate::ParserInput;
use crate::ast::{Location, SevenMarkElement, TextStyle};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn markdown_subscript_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_subscript {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.input.current_token_start();
    let parsed_content = delimited(
        literal(",,"),
        |input: &mut ParserInput| {
            input.state.set_subscript_context();
            let result = with_depth(input, element_parser);
            input.state.unset_subscript_context();
            result
        },
        literal(",,"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Subscript(TextStyle {
        location: Location { start, end },
        content: parsed_content,
    }))
}
