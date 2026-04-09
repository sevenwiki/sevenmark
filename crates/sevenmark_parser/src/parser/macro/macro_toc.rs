use crate::parser::ParserInput;
use sevenmark_ast::{Element, Span, TocElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse table-of-contents macro [toc]
pub fn macro_toc_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    literal("[toc]").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Toc(TocElement {
        span: Span { start, end },
    }))
}
