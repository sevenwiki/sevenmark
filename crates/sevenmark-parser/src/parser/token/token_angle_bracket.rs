use crate::ast::{Location, SevenMarkElement, TextElement};
use crate::parser::ParserInput;
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// `<` 토큰 파서: 멘션 파서가 실패한 경우 `<`를 Text로 변환
pub fn token_angle_bracket_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();
    literal("<").parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: "<".to_string(),
    }))
}
