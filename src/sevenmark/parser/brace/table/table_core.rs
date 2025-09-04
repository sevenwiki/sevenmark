use super::super::super::element::element_parser;
use super::super::super::parameter::parameter_core_parser;
use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{TableInnerElement1, TableInnerElement2};
use crate::sevenmark::parser::utils::{utils_get_common_style, with_depth};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, repeat};
use winnow::prelude::*;
use winnow::token::literal;

pub fn table_core_parser(parser_input: &mut ParserInput) -> Result<Vec<TableInnerElement1>> {
    repeat(1.., table_element_parser).parse_next(parser_input)
}

fn table_element_parser(parser_input: &mut ParserInput) -> Result<TableInnerElement1> {
    let (_, (parameters, parsed_content), _) = (
        multispace0,
        delimited(
            literal("[["),
            (
                opt(parameter_core_parser),
                repeat(1.., table_inner_element_parser),
            ),
            literal("]]"),
        ),
        multispace0,
    )
        .parse_next(parser_input)?;

    let common_style = utils_get_common_style(parameters.unwrap_or_default());

    Ok(TableInnerElement1 {
        common_style,
        inner_content: parsed_content,
    })
}

fn table_inner_element_parser(parser_input: &mut ParserInput) -> Result<TableInnerElement2> {
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

    let parameters = parameters.unwrap_or_default();

    // x, y
    let x = parameters.get("x").map(|p| p.value.clone()).unwrap_or_else(Vec::new);
    let y = parameters.get("y").map(|p| p.value.clone()).unwrap_or_else(Vec::new);

    let common_style = utils_get_common_style(parameters);

    Ok(TableInnerElement2 {
        common_style,
        x,
        y,
        content: parsed_content,
    })
}
