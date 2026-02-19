use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::expr::condition_parser;
use crate::parser::utils::with_depth_and_trim;
use sevenmark_ast::{Element, IfElement, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse if conditional elements: {{{#if condition :: content}}}
pub fn brace_if_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#if").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let condition = condition_parser.parse_next(parser_input)?;
    let parsed_content = with_depth_and_trim(parser_input, element_parser)?;

    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::If(IfElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        condition,
        children: parsed_content,
    }))
}
