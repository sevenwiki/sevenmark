use crate::parser::ParserInput;
use crate::parser::brace::category::category_content_parser;
use crate::parser::utils::with_depth;
use sevenmark_ast::{CategoryElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_category_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#category").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    multispace0.parse_next(parser_input)?;
    let parsed_content = with_depth(parser_input, category_content_parser)?;

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    // consume trailing whitespace to prevent unwanted line breaks
    multispace0.parse_next(parser_input)?;

    Ok(Element::Category(CategoryElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        children: parsed_content,
    }))
}
