use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use sevenmark_ast::{Element, Span, TeXElement};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_until};

/// Parse TeX elements enclosed in {{{#tex }}}
pub fn brace_tex_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#tex").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let parsed_content = take_until(0.., "}}}").parse_next(parser_input)?;

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    let is_block = parameters
        .as_ref()
        .map(|p| p.contains_key("block"))
        .unwrap_or(false);

    Ok(Element::TeX(TeXElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        is_block,
        value: parsed_content.to_string(),
    }))
}
