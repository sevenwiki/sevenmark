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

    input.state.decrease_depth();

    result
}

/// {{{ }}} 앞 trim 컨텍스트에서 파서를 실행
pub fn with_trim_brace<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    input.state.increase_trim_brace_depth();
    let result = parser(input);
    input.state.decrease_trim_brace_depth();
    result
}

/// [[ ]] 앞 trim 컨텍스트에서 파서를 실행
pub fn with_trim_bracket<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    input.state.increase_trim_bracket_depth();
    let result = parser(input);
    input.state.decrease_trim_bracket_depth();
    result
}

/// 깊이 관리 + brace trim 컨텍스트 ({{{ }}})
pub fn with_depth_and_trim_brace<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    with_depth(input, |inner| with_trim_brace(inner, parser))
}

/// 깊이 관리 + bracket trim 컨텍스트 ([[ ]])
pub fn with_depth_and_trim_bracket<T, F>(input: &mut ParserInput, parser: F) -> Result<T>
where
    F: FnOnce(&mut ParserInput) -> Result<T>,
{
    with_depth(input, |inner| with_trim_bracket(inner, parser))
}
