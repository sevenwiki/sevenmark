use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::ParserInput;
use winnow::prelude::*;
use winnow::token::literal;
use winnow::Result;

pub fn macro_newline_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    literal("[br]").parse_next(parser_input)?;

    Ok(SevenMarkElement::NewLine)
}
