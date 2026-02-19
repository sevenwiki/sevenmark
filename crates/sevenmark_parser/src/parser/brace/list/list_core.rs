use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::expr::expr_condition::condition_parser;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::with_depth_and_trim;
use sevenmark_ast::{ConditionalListItems, Expression, ListContentItem, ListItemElement, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{alt, opt, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// 리스트 내용 파서 - Vec<ListContentItem> 반환
pub fn list_core_parser(parser_input: &mut ParserInput) -> Result<Vec<ListContentItem>> {
    repeat(1.., list_item_child_parser).parse_next(parser_input)
}

/// 리스트 콘텐츠 아이템 파서 (아이템 또는 조건부)
fn list_item_child_parser(parser_input: &mut ParserInput) -> Result<ListContentItem> {
    alt((
        list_item_parser.map(ListContentItem::Item),
        list_conditional_parser.map(ListContentItem::Conditional),
    ))
    .parse_next(parser_input)
}

/// 리스트 아이템 레벨 조건부 파서 (전용 파서 - content가 리스트 item임)
/// {{{#if condition :: [[item1]] [[item2]] ... }}}
fn list_conditional_parser(parser_input: &mut ParserInput) -> Result<ConditionalListItems> {
    let start = parser_input.current_token_start();

    // {{{#if 시작
    multispace0.parse_next(parser_input)?;
    literal("{{{#if").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    // 조건식 파싱 (condition_parser는 선택적 :: 종결자를 처리함)
    let condition: Expression = condition_parser.parse_next(parser_input)?;

    // 리스트 아이템들 파싱 (0개 이상의 아이템)
    let items: Vec<ListItemElement> = repeat(0.., list_item_parser).parse_next(parser_input)?;

    // }}} 종료
    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(ConditionalListItems {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end: close_start + 3,
        },
        condition,
        items,
    })
}

/// 리스트 아이템 파서 - ListItemElement 반환
fn list_item_parser(parser_input: &mut ParserInput) -> Result<ListItemElement> {
    let start = parser_input.current_token_start();

    multispace0.parse_next(parser_input)?;
    literal("[[").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let parsed_content = with_depth_and_trim(parser_input, element_parser)?;

    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("]]").parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(ListItemElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end: close_start + 2,
        },
        parameters: parameters.unwrap_or_default(),
        children: parsed_content,
    })
}
