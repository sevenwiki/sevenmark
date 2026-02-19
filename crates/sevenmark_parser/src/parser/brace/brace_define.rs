use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use sevenmark_ast::{DefineElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse styled elements enclosed in {{{ }}}
pub fn brace_define_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#define").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = parameter_core_parser.parse_next(parser_input)?;

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    // consume trailing whitespace to prevent unwanted line breaks
    multispace0.parse_next(parser_input)?;

    Ok(Element::Define(DefineElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        parameters,
    }))
}
