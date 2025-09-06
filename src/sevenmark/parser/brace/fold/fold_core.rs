use super::super::super::element::element_parser;
use super::super::super::parameter::parameter_core_parser;
use crate::sevenmark::ast::FoldInnerElement;
use crate::sevenmark::parser::utils::with_depth;
use crate::sevenmark::ParserInput;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::token::literal;
use winnow::Result;

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

    Ok((
        FoldInnerElement {
            parameters: parameters_1,
            content: parsed_content_1,
        },
        FoldInnerElement {
            parameters: parameters_2,
            content: parsed_content_2,
        },
    ))
}
