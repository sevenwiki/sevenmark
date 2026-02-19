use sevenmark_ast::{Expression, LogicalOperator, LogicalOperatorKind, Span};
use crate::parser::ParserInput;
use crate::parser::expr::expr_comparison::comparison_parser;
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, repeat, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

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
    let start = input.current_token_start();
    let first = and_parser.parse_next(input)?;

    let rest: Vec<(LogicalOperator, Expression)> = repeat(
        0..,
        (
            delimited(multispace0, or_operator_parser, multispace0),
            and_parser,
        ),
    )
    .parse_next(input)?;

    let end = input.previous_token_end();

    Ok(rest
        .into_iter()
        .fold(first, |acc, (op, expr)| Expression::Or {
            span: Span { start, end },
            operator: op,
            left: Box::new(acc),
            right: Box::new(expr),
        }))
}

/// || 연산자 파서
fn or_operator_parser(input: &mut ParserInput) -> Result<LogicalOperator> {
    let start = input.current_token_start();
    literal("||").parse_next(input)?;
    let end = input.previous_token_end();

    Ok(LogicalOperator {
        span: Span { start, end },
        kind: LogicalOperatorKind::Or,
    })
}

/// AND 연산자 파서 (바인딩 파워 7)
fn and_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let first = not_parser.parse_next(input)?;

    let rest: Vec<(LogicalOperator, Expression)> = repeat(
        0..,
        (
            delimited(multispace0, and_operator_parser, multispace0),
            not_parser,
        ),
    )
    .parse_next(input)?;

    let end = input.previous_token_end();

    Ok(rest
        .into_iter()
        .fold(first, |acc, (op, expr)| Expression::And {
            span: Span { start, end },
            operator: op,
            left: Box::new(acc),
            right: Box::new(expr),
        }))
}

/// && 연산자 파서
fn and_operator_parser(input: &mut ParserInput) -> Result<LogicalOperator> {
    let start = input.current_token_start();
    literal("&&").parse_next(input)?;
    let end = input.previous_token_end();

    Ok(LogicalOperator {
        span: Span { start, end },
        kind: LogicalOperatorKind::And,
    })
}

/// NOT 연산자 파서 (바인딩 파워 15)
/// ! 하나만 허용. 이중 부정이 필요하면 !(!x) 형태로 작성
fn not_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();

    let not_op: Option<Span> =
        opt(terminated(not_operator_span_parser, multispace0)).parse_next(input)?;

    let inner = comparison_parser.parse_next(input)?;

    let end = input.previous_token_end();

    match not_op {
        Some(op_span) => Ok(Expression::Not {
            span: Span { start, end },
            operator: LogicalOperator {
                span: op_span,
                kind: LogicalOperatorKind::Not,
            },
            inner: Box::new(inner),
        }),
        None => Ok(inner),
    }
}

/// ! 연산자 위치 파서
fn not_operator_span_parser(input: &mut ParserInput) -> Result<Span> {
    let start = input.current_token_start();
    // != 연산자와 구분하기 위해 !뒤에 =가 없는지 확인
    (literal('!'), winnow::combinator::not(literal('='))).parse_next(input)?;
    let end = input.previous_token_end();

    Ok(Span { start, end })
}
