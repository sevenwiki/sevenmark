use crate::parser::ParserInput;
use crate::parser::brace::list::list_core_parser;
use crate::parser::parameter::parameter_core_parser;
use sevenmark_ast::{Element, ListElement, Span};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#list").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    let parsed_content = list_core_parser.parse_next(parser_input)?;

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    let parameters = parameters.unwrap_or_default();

    let kind = ["1", "a", "A", "i", "I"]
        .iter()
        .find(|&&k| parameters.contains_key(k))
        .map(|&k| k.to_string())
        .unwrap_or_default();

    Ok(Element::List(ListElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        kind,
        parameters,
        children: parsed_content,
    }))
}
