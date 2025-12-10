use crate::ast::{ListContentItem, ListInnerElement1, Location};
use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::expr::expr_condition::condition_parser;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::with_depth;
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{alt, delimited, opt, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn list_core_parser(parser_input: &mut ParserInput) -> Result<Vec<ListContentItem>> {
    repeat(1.., list_content_item_parser).parse_next(parser_input)
}

/// 리스트 콘텐츠 아이템 파서 (아이템 또는 조건부)
fn list_content_item_parser(parser_input: &mut ParserInput) -> Result<ListContentItem> {
    alt((
        list_element_parser.map(ListContentItem::Item),
        list_conditional_parser,
    ))
    .parse_next(parser_input)
}

/// 리스트 아이템 레벨 조건부 파서 (전용 파서 - content가 리스트 item임)
/// {{{#if condition :: [[item1]] [[item2]] ... }}}
fn list_conditional_parser(parser_input: &mut ParserInput) -> Result<ListContentItem> {
    let start = parser_input.input.current_token_start();

    // {{{#if 시작
    let _ = (multispace0, literal("{{{#if")).parse_next(parser_input)?;

    // 조건식 파싱 (condition_parser는 선택적 :: 종결자를 처리함)
    let condition = condition_parser.parse_next(parser_input)?;

    // 리스트 아이템들 파싱 (0개 이상의 아이템)
    let items: Vec<ListInnerElement1> =
        repeat(0.., list_element_parser).parse_next(parser_input)?;

    // }}} 종료
    let _ = (multispace0, literal("}}}"), multispace0).parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(ListContentItem::Conditional {
        location: Location { start, end },
        condition,
        items,
    })
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
