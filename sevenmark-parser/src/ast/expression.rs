use serde::Serialize;

use super::{Location, SevenMarkElement};

/// 비교 연산자
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ComparisonOperator {
    Equal,        // ==
    NotEqual,     // !=
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
}

/// 조건식 Expression AST
#[derive(Debug, Clone, Serialize)]
pub enum Expression {
    /// 논리 OR 연산
    Or(Box<Expression>, Box<Expression>),
    /// 논리 AND 연산
    And(Box<Expression>, Box<Expression>),
    /// 논리 NOT 연산
    Not(Box<Expression>),

    /// 비교 연산
    Comparison {
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },

    /// 함수 호출: int([var(x)]), len([var(str)])
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },

    /// 조건식 전용 리터럴
    StringLiteral(String),
    NumberLiteral(i64),
    BoolLiteral(bool),
    Null,

    /// 기존 SevenMarkElement 그대로 포함 (변환 없음)
    Element(Box<SevenMarkElement>),

    /// 괄호 그룹
    Group(Box<Expression>),
}

/// If 조건문 요소
#[derive(Debug, Clone, Serialize)]
pub struct IfElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub condition: Expression,
    pub content: Vec<SevenMarkElement>,
}