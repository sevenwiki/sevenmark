use crate::parser::ParserInput;
use crate::parser::utils::is_line_end_char;
use sevenmark_ast::{Element, Span, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;

/// 특수문자가 아닌 일반 텍스트 파싱 (기존 md_content_parser 역할)
pub fn text_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    let parsed_content = take_while(1.., |c: char| {
        !matches!(
            c,
            '*' | '~' | '_' | '^' | ',' | '{' | '}' | '[' | ']' | '/' | '\\' | '<'
        ) && !is_line_end_char(c)
    })
    .parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    Ok(Element::Text(TextElement {
        span: Span { start, end },
        value: parsed_content.to_string(),
    }))
}
