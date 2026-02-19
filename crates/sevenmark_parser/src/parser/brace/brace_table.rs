use crate::parser::ParserInput;
use crate::parser::brace::table::table_core_parser;
use crate::parser::parameter::parameter_core_parser;
use sevenmark_ast::{Element, Span, TableElement};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_table_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#table").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    let parsed_content = table_core_parser.parse_next(parser_input)?;

    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Table(TableElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        parameters: parameters.unwrap_or_default(),
        children: parsed_content,
    }))
}
