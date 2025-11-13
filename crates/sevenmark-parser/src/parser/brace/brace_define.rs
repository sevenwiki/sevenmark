use super::super::parameter::parameter_core_parser;
use crate::ast::SevenMarkElement;
use crate::{DefineElement, Location, ParserInput};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse styled elements enclosed in {{{ }}}
pub fn brace_define_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let parameters = delimited(literal("{{{#define"), parameter_core_parser, literal("}}}"))
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::DefineElement(DefineElement {
        location: Location { start, end },
        parameters,
    }))
}
