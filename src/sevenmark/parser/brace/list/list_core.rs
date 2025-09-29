use super::super::super::element::element_parser;
use super::super::super::parameter::parameter_core_parser;
use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::ListInnerElement1;
use crate::sevenmark::parser::utils::with_depth;
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, repeat};
use winnow::prelude::*;
use winnow::token::literal;

pub fn list_core_parser(parser_input: &mut ParserInput) -> Result<Vec<ListInnerElement1>> {
    repeat(1.., list_element_parser).parse_next(parser_input)
}

fn list_element_parser(parser_input: &mut ParserInput) -> Result<ListInnerElement1> {
    let (_, ((parameters, _), parsed_content), _) = (
        multispace0,
        delimited(
            literal("[["),
            (
                (opt(parameter_core_parser), multispace0),
                |input: &mut ParserInput| with_depth(input, element_parser),
            ),
            literal("]]"),
        ),
        multispace0,
    )
        .parse_next(parser_input)?;

    Ok(ListInnerElement1 {
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    })
}
