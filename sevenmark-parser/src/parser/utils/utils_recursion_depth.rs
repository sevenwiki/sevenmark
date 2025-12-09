use crate::parser::ParserInput;
use winnow::Result;

/// 깊이 관리가 포함된 파서를 실행하는 헬퍼 함수
pub fn with_depth<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    input
        .state
        .increase_depth()
        .map_err(|e| e.into_context_error())?;

    let result = parser(input);

    // increase_depth가 성공했으므로 decrease_depth도 항상 성공
    let _ = input.state.decrease_depth();

    result
}
