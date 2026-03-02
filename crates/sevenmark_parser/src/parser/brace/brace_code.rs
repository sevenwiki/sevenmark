use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::parse_raw_until_balanced_triple_brace;
use sevenmark_ast::{CodeElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_code_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#code").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let raw = parse_raw_until_balanced_triple_brace(parser_input)?;

    Ok(Element::Code(CodeElement {
        span: Span {
            start,
            end: raw.close_end,
        },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: raw.close_start,
            end: raw.close_end,
        },
        parameters: parameters.unwrap_or_default(),
        value: raw.value,
    }))
}
