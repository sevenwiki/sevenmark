use crate::parser::ParserInput;
use crate::parser::expr::expr_operand::operand_parser;
use sevenmark_ast::{ComparisonOperator, ComparisonOperatorKind, Expression, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{alt, delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// 비교 연산자 파서 (바인딩 파워 10)
pub fn comparison_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let left = operand_parser.parse_next(input)?;

    // 비교 연산자 + 오른쪽 피연산자 파싱 시도
    let op_and_right: Option<(ComparisonOperator, Expression)> = opt((
        delimited(multispace0, comparison_operator_parser, multispace0),
        operand_parser,
    ))
    .parse_next(input)?;

    let end = input.previous_token_end();

    match op_and_right {
        Some((op, right)) => Ok(Expression::Comparison {
            span: Span { start, end },
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }),
        None => Ok(left),
    }
}

/// 비교 연산자 파싱
fn comparison_operator_parser(input: &mut ParserInput) -> Result<ComparisonOperator> {
    let start = input.current_token_start();
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
    let end = input.previous_token_end();

    Ok(ComparisonOperator {
        span: Span { start, end },
        kind,
    })
}
