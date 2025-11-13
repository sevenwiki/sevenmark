use crate::ParserInput;
use winnow::Result;

/// 깊이 관리가 포함된 파서를 실행하는 헬퍼 함수
pub fn with_depth<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    let mut inner_input = input.clone();
    inner_input
        .state
        .increase_depth()
        .map_err(|e| e.into_context_error())?;

    let result = parser(&mut inner_input);

    inner_input
        .state
        .decrease_depth()
        .map_err(|e| e.into_context_error())?;

    *input = inner_input;
    result
}
