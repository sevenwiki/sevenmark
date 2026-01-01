use crate::ast::{AstNode, Location, NodeKind};
use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse styled elements enclosed in {{{ }}}
pub fn brace_define_parser(parser_input: &mut ParserInput) -> Result<AstNode> {
    let start = parser_input.input.current_token_start();

    let parameters = delimited(literal("{{{#define"), parameter_core_parser, literal("}}}"))
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(AstNode::new(
        Location { start, end },
        NodeKind::Define { parameters },
    ))
}