use super::super::element::element_parser;
use super::super::parameter::parameter_core_parser;
use crate::sevenmark::ast::{RubyElement, SevenMarkElement};
use crate::sevenmark::parser::utils::with_depth;
use crate::sevenmark::{Location, ParserInput};
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use winnow::Result;

/// Parse ruby elements enclosed in {{{#ruby }}}
pub fn brace_ruby_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#ruby"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| with_depth(input, element_parser),
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::RubyElement(RubyElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    }))
}