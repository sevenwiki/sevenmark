use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::parse_raw_until_line_closer;
use sevenmark_ast::{Element, Span, TeXElement};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse TeX elements enclosed in {{{#tex }}}
pub fn brace_tex_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#tex").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let raw = parse_raw_until_line_closer(parser_input, "}}}")?;

    let is_block = parameters
        .as_ref()
        .map(|p| p.contains_key("block"))
        .unwrap_or(false);

    Ok(Element::TeX(TeXElement {
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
        is_block,
        value: raw.value,
    }))
}
