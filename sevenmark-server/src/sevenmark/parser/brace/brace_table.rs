use super::super::parameter::parameter_core_parser;
use super::table::table_core_parser;
use crate::sevenmark::ast::{SevenMarkElement, TableElement};
use crate::sevenmark::{Location, ParserInput};
use winnow::Result;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_table_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (parameters, parsed_content) = delimited(
        literal("{{{#table"),
        (opt(parameter_core_parser), table_core_parser),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::TableElement(TableElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    }))
}
