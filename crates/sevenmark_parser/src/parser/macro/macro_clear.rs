use crate::parser::ParserInput;
use sevenmark_ast::{ClearElement, Element, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn macro_clear_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    literal("[clear]").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Clear(ClearElement {
        span: Span { start, end },
    }))
}
