use crate::parser::ParserInput;
use sevenmark_ast::{DateElement, Element, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn macro_date_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    literal("[date]").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Date(DateElement {
        span: Span { start, end },
    }))
}
