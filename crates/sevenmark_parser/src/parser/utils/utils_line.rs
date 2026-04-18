use crate::parser::ParserInput;
use winnow::Result;
use winnow::ascii::line_ending as winnow_line_ending;
use winnow::combinator::{eof, opt};
use winnow::prelude::*;
use winnow::token::take_till;

pub fn is_line_end_char(c: char) -> bool {
    c == '\n' || c == '\r'
}

pub fn line_content<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    take_till(0.., is_line_end_char).parse_next(parser_input)
}

pub fn line_break<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    winnow_line_ending.parse_next(parser_input)
}

pub fn line_end<'i>(parser_input: &mut ParserInput<'i>) -> Result<Option<&'i str>> {
    if let Some(ending) = opt(line_break).parse_next(parser_input)? {
        return Ok(Some(ending));
    }

    eof.parse_next(parser_input)?;
    Ok(None)
}
