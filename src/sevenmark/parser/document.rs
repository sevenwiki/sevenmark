use super::brace::brace_redirect_parser;
use super::element::element_parser;
use super::{InputSource, ParserInput};
use crate::sevenmark::ast::{SevenMarkElement, TextElement};
use crate::sevenmark::context::ParseContext;
use line_span::LineSpanExt;
use std::collections::HashSet;
use winnow::Result;
use winnow::combinator::repeat;
use winnow::prelude::*;
use winnow::stream::Location;

/// 문서 파서 - element들을 반복 파싱 (기존 many0 + alt 패턴)
/// redirect가 문서 시작에 있으면 그것만 반환하고 파싱 중단
pub fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    // 먼저 redirect 파서를 시도
    if let Ok(redirect_element) = brace_redirect_parser(parser_input) {
        return Ok(vec![redirect_element]);
    }

    // redirect가 아니면 기존처럼 모든 element 파싱
    repeat(0.., element_parser)
        .map(|elements: Vec<_>| elements.into_iter().flatten().collect())
        .parse_next(parser_input)
}
