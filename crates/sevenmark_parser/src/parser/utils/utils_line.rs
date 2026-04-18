use crate::parser::ParserInput;
use winnow::Result;
use winnow::combinator::{eof, opt};
use winnow::prelude::*;
use winnow::token::{literal, take_till};

/// Returns whether a character terminates a logical SevenMark line.
pub fn is_line_end_char(c: char) -> bool {
    c == '\n'
}

/// Parses the content portion of a line without consuming the line terminator.
pub fn line_content<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    take_till(0.., is_line_end_char).parse_next(parser_input)
}

/// Consumes one SevenMark line break token.
pub fn line_break<'i>(parser_input: &mut ParserInput<'i>) -> Result<&'i str> {
    literal("\n").parse_next(parser_input)
}

/// Consumes a line break when present, otherwise requires end of input.
pub fn line_break_or_eof<'i>(parser_input: &mut ParserInput<'i>) -> Result<Option<&'i str>> {
    if let Some(ending) = opt(line_break).parse_next(parser_input)? {
        return Ok(Some(ending));
    }

    eof.parse_next(parser_input)?;
    Ok(None)
}
