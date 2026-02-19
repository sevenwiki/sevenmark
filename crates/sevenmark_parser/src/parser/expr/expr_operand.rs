use crate::parser::ParserInput;
use crate::parser::expr::expr_condition::condition_parser;
use crate::parser::expr::expr_literal::{
    bool_literal_parser, null_parser, number_literal_parser, string_literal_parser,
};
use crate::parser::r#macro::macro_variable_parser;
use crate::parser::utils::with_depth;
use sevenmark_ast::{Expression, Span};
use winnow::Result;
use winnow::ascii::{alpha1, multispace0};
use winnow::combinator::{alt, delimited, separated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// 피연산자 파서
pub fn operand_parser(input: &mut ParserInput) -> Result<Expression> {
    alt((
        // 괄호 그룹
        group_parser,
        // 함수 호출: int(...), len(...)
        function_call_parser,
        // null 키워드
        null_parser,
        // bool 키워드: true, false
        bool_literal_parser,
        // 문자열 리터럴
        string_literal_parser,
        // 숫자 리터럴
        number_literal_parser,
        // 기존 매크로 파서들 (Element 반환 → Expression::Element로 래핑)
        macro_variable_parser.map(|e| Expression::Element(Box::new(e))),
    ))
    .parse_next(input)
}

/// 괄호 그룹 파서
fn group_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let inner = delimited(
        (literal('('), multispace0),
        |input: &mut ParserInput| with_depth(input, condition_parser),
        (multispace0, literal(')')),
    )
    .parse_next(input)?;
    let end = input.previous_token_end();

    Ok(Expression::Group {
        span: Span { start, end },
        inner: Box::new(inner),
    })
}

/// 함수 호출 파서: int(...), len(...), str(...)
fn function_call_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let name: &str = alpha1.parse_next(input)?;

    // 함수 이름 검증
    if !matches!(name, "int" | "len" | "str") {
        return Err(winnow::error::ContextError::new());
    }

    let arguments: Vec<Expression> = delimited(
        (literal('('), multispace0),
        separated(
            0..,
            |input: &mut ParserInput| with_depth(input, condition_parser),
            (multispace0, literal(','), multispace0),
        ),
        (multispace0, literal(')')),
    )
    .parse_next(input)?;

    let end = input.previous_token_end();

    Ok(Expression::FunctionCall {
        span: Span { start, end },
        name: name.to_string(),
        arguments,
    })
}
