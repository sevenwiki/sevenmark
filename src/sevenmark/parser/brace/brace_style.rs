use super::super::element::element_parser;
use super::super::parameter::parameter_core_parser;
use crate::sevenmark::ast::{SevenMarkElement, StyledElement};
use crate::sevenmark::parser::utils::{utils_get_common_style, with_depth};
use crate::sevenmark::{Location, ParserInput};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse styled elements enclosed in {{{ }}}
pub fn brace_style_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (parameters, parsed_content) = delimited(
        literal("{{{"),
        (parameter_core_parser, |input: &mut ParserInput| {
            with_depth(input, element_parser)
        }),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    let common_style = utils_get_common_style(parameters);

    Ok(SevenMarkElement::StyledElement(StyledElement {
        location: Location { start, end },
        common_style,
        content: parsed_content,
    }))
}
