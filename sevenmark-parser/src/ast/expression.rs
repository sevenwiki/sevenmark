use serde::Serialize;

use super::{Location, SevenMarkElement};

/// 비교 연산자 종류
#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ComparisonOperatorKind {
    Equal,        // ==
    NotEqual,     // !=
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
}

/// 비교 연산자 (위치 정보 포함)
#[derive(Debug, Clone, Serialize)]
pub struct ComparisonOperator {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub kind: ComparisonOperatorKind,
}

/// 조건식 Expression AST
#[derive(Debug, Clone, Serialize)]
pub enum Expression {
    /// 논리 OR 연산
    Or {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// 논리 AND 연산
    And {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// 논리 NOT 연산
    Not {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        inner: Box<Expression>,
    },

    /// 비교 연산
    Comparison {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        left: Box<Expression>,
        operator: ComparisonOperator,
        right: Box<Expression>,
    },

    /// 함수 호출: int([var(x)]), len([var(str)])
    FunctionCall {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        name: String,
        arguments: Vec<Expression>,
    },

    /// 조건식 전용 리터럴
    StringLiteral {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        value: String,
    },
    NumberLiteral {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        value: i64,
    },
    BoolLiteral {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        value: bool,
    },
    Null {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
    },

    /// 기존 SevenMarkElement 그대로 포함 (변환 없음, 자체 location 보유)
    Element(Box<SevenMarkElement>),

    /// 괄호 그룹
    Group {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        inner: Box<Expression>,
    },
}

/// If 조건문 요소
#[derive(Debug, Clone, Serialize)]
pub struct IfElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub condition: Expression,
    pub content: Vec<SevenMarkElement>,
}