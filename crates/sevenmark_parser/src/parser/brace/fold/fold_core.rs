use super::super::super::element::element_parser;
use super::super::super::parameter::parameter_core_parser;

use crate::parser::ParserInput;
use crate::parser::utils::with_depth_and_trim;
use sevenmark_ast::{FoldInnerElement, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Fold 내용 파서 - (FoldInnerElement, FoldInnerElement) 튜플 반환
pub fn fold_core_parser(
    parser_input: &mut ParserInput,
) -> Result<(FoldInnerElement, FoldInnerElement)> {
    // 첫 번째 FoldInner
    let start_1 = parser_input.current_token_start();
    multispace0.parse_next(parser_input)?;
    let open_start_1 = parser_input.current_token_start();
    literal("[[").parse_next(parser_input)?;
    let open_end_1 = parser_input.previous_token_end();

    let parameters_1 = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let parsed_content_1 = with_depth_and_trim(parser_input, element_parser)?;

    multispace0.parse_next(parser_input)?;
    let close_start_1 = parser_input.current_token_start();
    literal("]]").parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let end_1 = parser_input.previous_token_end();

    // 두 번째 FoldInner
    let start_2 = parser_input.current_token_start();
    multispace0.parse_next(parser_input)?;
    let open_start_2 = parser_input.current_token_start();
    literal("[[").parse_next(parser_input)?;
    let open_end_2 = parser_input.previous_token_end();

    let parameters_2 = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let parsed_content_2 = with_depth_and_trim(parser_input, element_parser)?;

    multispace0.parse_next(parser_input)?;
    let close_start_2 = parser_input.current_token_start();
    literal("]]").parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let end_2 = parser_input.previous_token_end();

    let fold_inner_1 = FoldInnerElement {
        span: Span {
            start: start_1,
            end: end_1,
        },
        open_span: Span {
            start: open_start_1,
            end: open_end_1,
        },
        close_span: Span {
            start: close_start_1,
            end: close_start_1 + 2,
        },
        parameters: parameters_1.unwrap_or_default(),
        children: parsed_content_1,
    };

    let fold_inner_2 = FoldInnerElement {
        span: Span {
            start: start_2,
            end: end_2,
        },
        open_span: Span {
            start: open_start_2,
            end: open_end_2,
        },
        close_span: Span {
            start: close_start_2,
            end: close_start_2 + 2,
        },
        parameters: parameters_2.unwrap_or_default(),
        children: parsed_content_2,
    };

    Ok((fold_inner_1, fold_inner_2))
}
