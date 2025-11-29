use crate::ast::{ComparisonOperator, ComparisonOperatorKind, Expression, Location};
use crate::parser::ParserInput;
use crate::parser::r#macro::macro_variable_parser;
use winnow::Result;
use winnow::ascii::{alpha1, digit1, multispace0};
use winnow::combinator::{alt, delimited, opt, preceded, repeat, separated, terminated};
use winnow::prelude::*;
use winnow::token::{literal, one_of, take_while};
use winnow::stream::Location as StreamLocation;

/// 조건식 파서 (최상위)
/// 우선순위: OR < AND < NOT < Comparison < Operand
/// 선택적 "::" 종결자로 조건식 끝 표시 가능
pub fn condition_parser(input: &mut ParserInput) -> Result<Expression> {
    delimited(
        multispace0,
        or_parser,
        (multispace0, opt(literal("::")), multispace0),
    )
    .parse_next(input)
}

/// OR 연산자 파서 (최저 우선순위, 바인딩 파워 5)
fn or_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let first = and_parser.parse_next(input)?;

    let rest: Vec<Expression> = repeat(
        0..,
        preceded((multispace0, literal("||"), multispace0), and_parser),
    )
    .parse_next(input)?;

    let end = input.input.previous_token_end();

    Ok(rest.into_iter().fold(first, |acc, expr| {
        Expression::Or {
            location: Location { start, end },
            left: Box::new(acc),
            right: Box::new(expr),
        }
    }))
}

/// AND 연산자 파서 (바인딩 파워 7)
fn and_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let first = not_parser.parse_next(input)?;

    let rest: Vec<Expression> = repeat(
        0..,
        preceded((multispace0, literal("&&"), multispace0), not_parser),
    )
    .parse_next(input)?;

    let end = input.input.previous_token_end();

    Ok(rest.into_iter().fold(first, |acc, expr| {
        Expression::And {
            location: Location { start, end },
            left: Box::new(acc),
            right: Box::new(expr),
        }
    }))
}

/// NOT 연산자 파서 (바인딩 파워 15)
fn not_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let not_ops: Vec<_> = repeat(0.., terminated(literal('!'), multispace0)).parse_next(input)?;

    let inner = comparison_parser.parse_next(input)?;

    let end = input.input.previous_token_end();

    // 홀수 개의 !는 NOT, 짝수 개는 원래 값
    Ok(if not_ops.len() % 2 == 1 {
        Expression::Not {
            location: Location { start, end },
            inner: Box::new(inner),
        }
    } else {
        inner
    })
}

/// 비교 연산자 파서 (바인딩 파워 10)
fn comparison_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let left = operand_parser.parse_next(input)?;

    // 비교 연산자 + 오른쪽 피연산자 파싱 시도
    let op_and_right: Option<(ComparisonOperator, Expression)> = opt((
        delimited(multispace0, comparison_operator_parser, multispace0),
        operand_parser,
    ))
    .parse_next(input)?;

    let end = input.input.previous_token_end();

    match op_and_right {
        Some((op, right)) => Ok(Expression::Comparison {
            location: Location { start, end },
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }),
        None => Ok(left),
    }
}

/// 비교 연산자 파싱
fn comparison_operator_parser(input: &mut ParserInput) -> Result<ComparisonOperator> {
    let start = input.input.current_token_start();
    let kind = alt((
        // 2문자 연산자를 먼저 시도 (>= 보다 > 먼저 매칭되는 것 방지)
        literal("==").value(ComparisonOperatorKind::Equal),
        literal("!=").value(ComparisonOperatorKind::NotEqual),
        literal(">=").value(ComparisonOperatorKind::GreaterEqual),
        literal("<=").value(ComparisonOperatorKind::LessEqual),
        literal(">").value(ComparisonOperatorKind::GreaterThan),
        literal("<").value(ComparisonOperatorKind::LessThan),
    ))
    .parse_next(input)?;
    let end = input.input.previous_token_end();

    Ok(ComparisonOperator {
        location: Location { start, end },
        kind,
    })
}

/// 피연산자 파서
fn operand_parser(input: &mut ParserInput) -> Result<Expression> {
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
        // 기존 매크로 파서들 (Element로 감싸기)
        macro_variable_parser.map(|elem| Expression::Element(Box::new(elem))),
    ))
    .parse_next(input)
}

/// 괄호 그룹 파서
fn group_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let inner = delimited(
        (literal('('), multispace0),
        condition_parser,
        (multispace0, literal(')')),
    )
    .parse_next(input)?;
    let end = input.input.previous_token_end();

    Ok(Expression::Group {
        location: Location { start, end },
        inner: Box::new(inner),
    })
}

/// 함수 호출 파서: int(...), len(...), str(...)
fn function_call_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let name: &str = alpha1.parse_next(input)?;

    // 함수 이름 검증
    if !matches!(name, "int" | "len" | "str") {
        return Err(winnow::error::ContextError::new());
    }

    let arguments = delimited(
        (literal('('), multispace0),
        separated(
            0..,
            condition_parser,
            (multispace0, literal(','), multispace0),
        ),
        (multispace0, literal(')')),
    )
    .parse_next(input)?;

    let end = input.input.previous_token_end();

    Ok(Expression::FunctionCall {
        location: Location { start, end },
        name: name.to_string(),
        arguments,
    })
}

/// null 키워드 파서
fn null_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    literal("null").parse_next(input)?;
    let end = input.input.previous_token_end();

    Ok(Expression::Null {
        location: Location { start, end },
    })
}

/// bool 리터럴 파서: true, false
fn bool_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let value = alt((literal("true").value(true), literal("false").value(false))).parse_next(input)?;
    let end = input.input.previous_token_end();

    Ok(Expression::BoolLiteral {
        location: Location { start, end },
        value,
    })
}

/// 문자열 리터럴 파서: "..."
fn string_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let value = delimited(
        literal('"'),
        take_while(0.., |c| c != '"').map(|s: &str| s.to_string()),
        literal('"'),
    )
    .parse_next(input)?;
    let end = input.input.previous_token_end();

    Ok(Expression::StringLiteral {
        location: Location { start, end },
        value,
    })
}

/// 숫자 리터럴 파서
fn number_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.input.current_token_start();
    let sign = opt(one_of(['+', '-'])).parse_next(input)?;
    let digits: &str = digit1.parse_next(input)?;
    let end = input.input.previous_token_end();

    // digit1이 성공했으면 parse()도 반드시 성공
    let value: i64 = digits.parse().unwrap_or(0);

    let final_value = match sign {
        Some('-') => -value,
        _ => value,
    };

    Ok(Expression::NumberLiteral {
        location: Location { start, end },
        value: final_value,
    })
}
