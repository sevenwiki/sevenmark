use super::super::element::element_parser;
use super::super::utils::with_depth;
use crate::sevenmark::ast::{Location, SevenMarkElement, TextStyle};
use crate::sevenmark::ParserInput;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use winnow::Result;

pub fn markdown_italic_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_italic {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.input.current_token_start();
    let parsed_content = delimited(
        literal("*"),
        |input: &mut ParserInput| {
            input.state.set_italic_context();
            let result = with_depth(input, element_parser);
            input.state.unset_italic_context();
            result
        },
        literal("*"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Italic(TextStyle {
        location: Location { start, end },
        content: parsed_content,
    }))
}
