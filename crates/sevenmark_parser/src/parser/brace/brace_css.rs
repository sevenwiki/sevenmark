use crate::parser::ParserInput;
use crate::parser::utils::parse_raw_until_balanced_triple_brace;
use sevenmark_ast::{CssElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_css_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#css").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    multispace0.parse_next(parser_input)?;
    let raw = parse_raw_until_balanced_triple_brace(parser_input)?;

    Ok(Element::Css(CssElement {
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
        value: raw.value,
    }))
}
