use crate::parser::ParserInput;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::{literal, take_while};

/// ISO 8601 date parser (YYYY-MM-DD)
pub fn parse_date(parser_input: &mut ParserInput) -> Result<String> {
    let year = take_while(4..=4, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;
    literal("-").parse_next(parser_input)?;
    let month = take_while(2..=2, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;
    literal("-").parse_next(parser_input)?;
    let day = take_while(2..=2, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;

    Ok(format!("{}-{}-{}", year, month, day))
}
