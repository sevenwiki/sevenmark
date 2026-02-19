use super::string_literal::string_literal_content::string_literal_content_parser;
use sevenmark_ast::{Expression, Span};
use crate::parser::ParserInput;
use winnow::Result;
use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, one_of};

/// null 키워드 파서
pub fn null_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    literal("null").parse_next(input)?;
    let end = input.previous_token_end();

    Ok(Expression::Null {
        span: Span { start, end },
    })
}

/// bool 리터럴 파서: true, false
pub fn bool_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let value =
        alt((literal("true").value(true), literal("false").value(false))).parse_next(input)?;
    let end = input.previous_token_end();

    Ok(Expression::BoolLiteral {
        span: Span { start, end },
        value,
    })
}

/// 문자열 리터럴 파서: "..." (이스케이프: \", \\ 등)
pub fn string_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let value =
        delimited(literal('"'), string_literal_content_parser, literal('"')).parse_next(input)?;
    let end = input.previous_token_end();

    Ok(Expression::StringLiteral {
        span: Span { start, end },
        value,
    })
}

/// 숫자 리터럴 파서
pub fn number_literal_parser(input: &mut ParserInput) -> Result<Expression> {
    let start = input.current_token_start();
    let sign = opt(one_of(['+', '-'])).parse_next(input)?;
    let digits: &str = digit1.parse_next(input)?;
    let end = input.previous_token_end();

    // digit1이 성공했으면 parse()도 반드시 성공
    let value: i64 = digits.parse().unwrap_or(0);

    let final_value = match sign {
        Some('-') => -value,
        _ => value,
    };

    Ok(Expression::NumberLiteral {
        span: Span { start, end },
        value: final_value,
    })
}
