use super::string_literal_text::string_literal_text_parser;
use crate::ast::Element;
use crate::parser::ParserInput;
use crate::parser::escape::escape_parser;
use winnow::Result;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

/// 문자열 리터럴 내부 콘텐츠 파서 (escape + text)
pub fn string_literal_content_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    repeat(0.., alt((escape_parser, string_literal_text_parser))).parse_next(parser_input)
}
