use crate::parser::ParserInput;
use crate::parser::block_document_parser;
use crate::parser::brace::brace_redirect_parser;
use sevenmark_ast::Element;
use winnow::Result;

/// 문서 파서 - element들을 반복 파싱 (기존 many0 + alt 패턴)
/// redirect가 문서 시작에 있으면 그것만 반환하고 파싱 중단
pub fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    // 문서 시작이 redirect라면 redirect만 허용한다.
    // 실패는 곧 문서 파싱 실패로 전파해 상위에서 Error element로 처리한다.
    if parser_input.input.starts_with("{{{#redirect") {
        return brace_redirect_parser(parser_input).map(|redirect| vec![redirect]);
    }

    block_document_parser(parser_input)
}
