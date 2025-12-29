use crate::ast::SevenMarkElement;
use crate::parser::ParserInput;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::literal;

pub fn token_newline_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_header && parser_input.input.starts_with('\n') {
        return Err(winnow::error::ContextError::new());
    }

    literal("\n").parse_next(parser_input)?;

    Ok(SevenMarkElement::NewLine)
}
