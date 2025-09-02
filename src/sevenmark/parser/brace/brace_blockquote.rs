use super::super::element::element_parser;
use super::super::parameter::parameter_core_parser;
use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::parser::utils::{utils_get_common_style, with_depth};
use crate::sevenmark::{BlockQuoteElement, Location, ParserInput};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_blockquote_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#quote"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| with_depth(input, element_parser),
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    let common_style = utils_get_common_style(parameters.unwrap_or_default());

    Ok(SevenMarkElement::BlockQuoteElement(BlockQuoteElement {
        location: Location { start, end },
        common_style,
        content: parsed_content,
    }))
}
