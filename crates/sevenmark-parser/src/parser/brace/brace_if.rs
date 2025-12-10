use crate::ast::{IfElement, Location, SevenMarkElement};
use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::expr::condition_parser;
use crate::parser::utils::with_depth;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse if conditional elements: {{{#if condition :: content}}}
pub fn brace_if_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (condition, parsed_content) = delimited(
        literal("{{{#if"),
        (condition_parser, |input: &mut ParserInput| {
            with_depth(input, element_parser)
        }),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::IfElement(IfElement {
        location: Location { start, end },
        condition,
        content: parsed_content,
    }))
}
