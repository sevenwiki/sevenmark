use crate::parser::ParserInput;
use crate::parser::brace::brace_redirect_parser;
use crate::parser::element::element_parser;
use sevenmark_ast::Element;
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;

/// 문서 파서 - element들을 반복 파싱 (기존 many0 + alt 패턴)
/// redirect가 문서 시작에 있으면 그것만 반환하고 파싱 중단
pub fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    // 문서 시작이 redirect라면 redirect만 허용한다.
    // 실패는 곧 문서 파싱 실패로 전파해 상위에서 Error element로 처리한다.
    if parser_input.input.starts_with("{{{#redirect") {
        return brace_redirect_parser(parser_input).map(|redirect| vec![redirect]);
    }

    // redirect가 아니면 기존처럼 모든 element 파싱
    opt(element_parser)
        .map(|elements| elements.unwrap_or_default())
        .parse_next(parser_input)
}
