use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::literal;

pub fn macro_footnote_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    literal("[fn]").parse_next(parser_input)?;

    Ok(SevenMarkElement::FootNote)
}
