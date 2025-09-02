use super::super::super::element::element_parser;
use super::super::super::parameter::parameter_core_parser;
use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::FoldInnerElement;
use crate::sevenmark::parser::utils::{utils_get_common_style, with_depth};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::token::literal;

pub fn fold_core_parser(
    parser_input: &mut ParserInput,
) -> Result<(FoldInnerElement, FoldInnerElement)> {
    let (_, ((parameters_1, _), parsed_content_1), _, ((parameters_2, _), parsed_content_2), _) = (
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

    let parameters_1 = parameters_1.unwrap_or_default();
    let parameters_2 = parameters_2.unwrap_or_default();

    let common_style_1 = utils_get_common_style(parameters_1);
    let common_style_2 = utils_get_common_style(parameters_2);

    Ok((
        FoldInnerElement {
            common_style: common_style_1,
            content: parsed_content_1,
        },
        FoldInnerElement {
            common_style: common_style_2,
            content: parsed_content_2,
        },
    ))
}
