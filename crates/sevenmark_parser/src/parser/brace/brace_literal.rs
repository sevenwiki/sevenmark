use crate::parser::ParserInput;
use crate::parser::brace::literal::literal_content_parser;
use crate::parser::utils::with_depth_and_trim_brace;
use sevenmark_ast::{Element, LiteralElement, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse literal elements enclosed in {{{ }}}
pub fn brace_literal_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    multispace0.parse_next(parser_input)?;
    let parsed_content = with_depth_and_trim_brace(parser_input, literal_content_parser)?;

    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Literal(LiteralElement {
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
