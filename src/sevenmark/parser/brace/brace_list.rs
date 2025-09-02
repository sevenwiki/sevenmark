use super::super::parameter::parameter_core_parser;
use super::list::list_core_parser;
use crate::sevenmark::ast::{ListElement, SevenMarkElement};
use crate::sevenmark::parser::utils::utils_get_common_style;
use crate::sevenmark::{Location, ParserInput};
use winnow::Result;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_list_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (parameters, parsed_content) = delimited(
        literal("{{{#list"),
        (opt(parameter_core_parser), list_core_parser),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    let parameters = parameters.unwrap_or_default();

    let kind = ["1", "a", "A", "i", "I"]
        .iter()
        .find(|&&k| parameters.contains_key(k))
        .map(|&k| k.to_string())
        .unwrap_or_default();

    let common_style = utils_get_common_style(parameters);

    Ok(SevenMarkElement::ListElement(ListElement {
        location: Location { start, end },
        kind,
        common_style,
        content: parsed_content,
    }))
}
