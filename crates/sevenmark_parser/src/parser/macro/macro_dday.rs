use crate::parser::ParserInput;
use sevenmark_ast::{DdayElement, Element, Span};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

use super::utils::parse_date;

pub fn macro_dday_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    let date = delimited(literal("[dday("), parse_date, literal(")]")).parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(Element::Dday(DdayElement {
        span: Span { start, end },
        date,
    }))
}
