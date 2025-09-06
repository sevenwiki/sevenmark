use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::ParserInput;

use super::category_text::category_text_parser;
use crate::sevenmark::parser::escape::escape::escape_parser;
use crate::sevenmark::parser::token::{token_brace_close_parser, token_brace_open_parser};
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;
use winnow::Result;

pub fn category_content_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    repeat(
        1..,
        alt((
            escape_parser,
            // category text
            category_text_parser,
            // Token
            token_brace_open_parser,
            token_brace_close_parser,
        )),
    )
    .parse_next(parser_input)
}
