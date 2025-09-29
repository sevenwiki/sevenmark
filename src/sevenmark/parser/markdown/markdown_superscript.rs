use super::super::element::element_parser;
use super::super::utils::with_depth;
use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{Location, SevenMarkElement, TextStyle};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn markdown_superscript_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_superscript {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.input.current_token_start();
    let parsed_content = delimited(
        literal("^^"),
        |input: &mut ParserInput| {
            input.state.set_superscript_context();
            let result = with_depth(input, element_parser);
            input.state.unset_superscript_context();
            result
        },
        literal("^^"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Superscript(TextStyle {
        location: Location { start, end },
        content: parsed_content,
    }))
}
