use sevenmark_parser::ast::{ComparisonOperator, Expression, SevenMarkElement};
use std::collections::HashMap;

/// 조건식 평가 결과
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(i64),
    Null,
    Bool(bool),
}

/// 조건식을 평가하여 bool 반환
pub fn evaluate_condition(expr: &Expression, variables: &HashMap<String, String>) -> bool {
    match evaluate_expression(expr, variables) {
        Value::Bool(b) => b,
        Value::Null => false,
        Value::String(s) => !s.is_empty(),
        Value::Number(n) => n != 0,
    }
}

/// Expression을 Value로 평가
fn evaluate_expression(expr: &Expression, variables: &HashMap<String, String>) -> Value {
    match expr {
        // Short-circuit evaluation: true || X → true (X not evaluated)
        Expression::Or(left, right) => {
            if evaluate_condition(left, variables) {
                return Value::Bool(true);
            }
            Value::Bool(evaluate_condition(right, variables))
        }
        // Short-circuit evaluation: false && X → false (X not evaluated)
        Expression::And(left, right) => {
            if !evaluate_condition(left, variables) {
                return Value::Bool(false);
            }
            Value::Bool(evaluate_condition(right, variables))
        }
        Expression::Not(inner) => {
            let inner_val = evaluate_condition(inner, variables);
            Value::Bool(!inner_val)
        }
        Expression::Comparison {
            left,
            operator,
            right,
        } => {
            let left_val = evaluate_expression(left, variables);
            let right_val = evaluate_expression(right, variables);
            Value::Bool(compare_values(&left_val, operator, &right_val))
        }
        Expression::FunctionCall { name, arguments } => {
            evaluate_function(name, arguments, variables)
        }
        Expression::StringLiteral(s) => Value::String(s.clone()),
        Expression::NumberLiteral(n) => Value::Number(*n),
        Expression::BoolLiteral(b) => Value::Bool(*b),
        Expression::Null => Value::Null,
        Expression::Element(elem) => evaluate_element(elem, variables),
        Expression::Group(inner) => evaluate_expression(inner, variables),
    }
}

/// SevenMarkElement를 Value로 평가
fn evaluate_element(elem: &SevenMarkElement, variables: &HashMap<String, String>) -> Value {
    match elem {
        SevenMarkElement::Variable(var) => {
            if let Some(value) = variables.get(&var.content) {
                Value::String(value.clone())
            } else {
                Value::Null
            }
        }
        SevenMarkElement::Text(text) => Value::String(text.content.clone()),
        SevenMarkElement::Null => Value::Null,
        _ => Value::Null,
    }
}

/// 두 값 비교
fn compare_values(left: &Value, operator: &ComparisonOperator, right: &Value) -> bool {
    match operator {
        ComparisonOperator::Equal => values_equal(left, right),
        ComparisonOperator::NotEqual => !values_equal(left, right),
        // 숫자 비교는 양쪽 모두 숫자로 변환 가능할 때만 수행
        ComparisonOperator::GreaterThan => compare_numeric(left, right).is_some_and(|ord| ord > 0),
        ComparisonOperator::LessThan => compare_numeric(left, right).is_some_and(|ord| ord < 0),
        ComparisonOperator::GreaterEqual => {
            compare_numeric(left, right).is_some_and(|ord| ord >= 0)
        }
        ComparisonOperator::LessEqual => compare_numeric(left, right).is_some_and(|ord| ord <= 0),
    }
}

/// 값 동등 비교
fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
            s.parse::<i64>().map(|parsed| parsed == *n).unwrap_or(false)
        }
        _ => false,
    }
}

/// 숫자 비교 - 양쪽 모두 숫자로 변환 가능할 때만 비교
/// 변환 불가능하면 None 반환
fn compare_numeric(left: &Value, right: &Value) -> Option<i64> {
    let left_num = to_number(left)?;
    let right_num = to_number(right)?;
    Some((left_num - right_num).signum())
}

/// Value를 숫자로 변환 시도 (실패하면 None)
fn to_number(value: &Value) -> Option<i64> {
    match value {
        Value::Number(n) => Some(*n),
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(if *b { 1 } else { 0 }),
        Value::Null => None,
    }
}

/// 함수 호출 평가
fn evaluate_function(
    name: &str,
    arguments: &[Expression],
    variables: &HashMap<String, String>,
) -> Value {
    match name {
        "int" => {
            if let Some(arg) = arguments.first() {
                let val = evaluate_expression(arg, variables);
                Value::Number(to_number(&val).unwrap_or(0))
            } else {
                Value::Number(0)
            }
        }
        "len" => {
            if let Some(arg) = arguments.first() {
                let val = evaluate_expression(arg, variables);
                match val {
                    Value::String(s) => Value::Number(s.len() as i64),
                    Value::Null => Value::Number(0),
                    _ => Value::Number(0),
                }
            } else {
                Value::Number(0)
            }
        }
        "str" => {
            if let Some(arg) = arguments.first() {
                let val = evaluate_expression(arg, variables);
                match val {
                    Value::String(s) => Value::String(s),
                    Value::Number(n) => Value::String(n.to_string()),
                    Value::Null => Value::String(String::new()),
                    Value::Bool(b) => Value::String(b.to_string()),
                }
            } else {
                Value::String(String::new())
            }
        }
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_comparison() {
        let variables = HashMap::new();
        let expr = Expression::Comparison {
            left: Box::new(Expression::StringLiteral("hello".to_string())),
            operator: ComparisonOperator::Equal,
            right: Box::new(Expression::StringLiteral("hello".to_string())),
        };
        assert!(evaluate_condition(&expr, &variables));
    }

    #[test]
    fn test_variable_null_check() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());

        // [var(name)] != null → true
        let expr = Expression::Comparison {
            left: Box::new(Expression::Element(Box::new(SevenMarkElement::Variable(
                sevenmark_parser::ast::VariableElement {
                    location: sevenmark_parser::ast::Location::synthesized(),
                    content: "name".to_string(),
                },
            )))),
            operator: ComparisonOperator::NotEqual,
            right: Box::new(Expression::Null),
        };
        assert!(evaluate_condition(&expr, &variables));

        // [var(unknown)] != null → false
        let expr2 = Expression::Comparison {
            left: Box::new(Expression::Element(Box::new(SevenMarkElement::Variable(
                sevenmark_parser::ast::VariableElement {
                    location: sevenmark_parser::ast::Location::synthesized(),
                    content: "unknown".to_string(),
                },
            )))),
            operator: ComparisonOperator::NotEqual,
            right: Box::new(Expression::Null),
        };
        assert!(!evaluate_condition(&expr2, &variables));
    }

    #[test]
    fn test_and_or() {
        let variables = HashMap::new();

        // true && false → false
        let expr = Expression::And(
            Box::new(Expression::StringLiteral("yes".to_string())),
            Box::new(Expression::Null),
        );
        assert!(!evaluate_condition(&expr, &variables));

        // true || false → true
        let expr2 = Expression::Or(
            Box::new(Expression::StringLiteral("yes".to_string())),
            Box::new(Expression::Null),
        );
        assert!(evaluate_condition(&expr2, &variables));
    }

    #[test]
    fn test_numeric_comparison() {
        let variables = HashMap::new();

        let expr = Expression::Comparison {
            left: Box::new(Expression::NumberLiteral(10)),
            operator: ComparisonOperator::GreaterThan,
            right: Box::new(Expression::NumberLiteral(5)),
        };
        assert!(evaluate_condition(&expr, &variables));
    }

    #[test]
    fn test_int_function() {
        let mut variables = HashMap::new();
        variables.insert("age".to_string(), "25".to_string());

        let expr = Expression::Comparison {
            left: Box::new(Expression::FunctionCall {
                name: "int".to_string(),
                arguments: vec![Expression::Element(Box::new(SevenMarkElement::Variable(
                    sevenmark_parser::ast::VariableElement {
                        location: sevenmark_parser::ast::Location::synthesized(),
                        content: "age".to_string(),
                    },
                )))],
            }),
            operator: ComparisonOperator::GreaterEqual,
            right: Box::new(Expression::NumberLiteral(18)),
        };
        assert!(evaluate_condition(&expr, &variables));
    }

    #[test]
    fn test_short_circuit_and() {
        let variables = HashMap::new();

        // false && X → false (X not evaluated)
        // This pattern: [var(x)] != null && int([var(x)]) > 5
        // If x is null, int([var(x)]) should NOT be evaluated
        let expr = Expression::And(
            Box::new(Expression::Null), // false
            Box::new(Expression::FunctionCall {
                name: "int".to_string(),
                arguments: vec![Expression::Null],
            }),
        );
        assert!(!evaluate_condition(&expr, &variables));
    }

    #[test]
    fn test_short_circuit_or() {
        let variables = HashMap::new();

        // true || X → true (X not evaluated)
        let expr = Expression::Or(
            Box::new(Expression::StringLiteral("truthy".to_string())), // true
            Box::new(Expression::Null),                                // not evaluated
        );
        assert!(evaluate_condition(&expr, &variables));
    }

    #[test]
    fn test_null_guard_pattern() {
        let mut variables = HashMap::new();
        variables.insert("count".to_string(), "10".to_string());

        // [var(count)] != null && int([var(count)]) > 5 → true
        let expr = Expression::And(
            Box::new(Expression::Comparison {
                left: Box::new(Expression::Element(Box::new(SevenMarkElement::Variable(
                    sevenmark_parser::ast::VariableElement {
                        location: sevenmark_parser::ast::Location::synthesized(),
                        content: "count".to_string(),
                    },
                )))),
                operator: ComparisonOperator::NotEqual,
                right: Box::new(Expression::Null),
            }),
            Box::new(Expression::Comparison {
                left: Box::new(Expression::FunctionCall {
                    name: "int".to_string(),
                    arguments: vec![Expression::Element(Box::new(SevenMarkElement::Variable(
                        sevenmark_parser::ast::VariableElement {
                            location: sevenmark_parser::ast::Location::synthesized(),
                            content: "count".to_string(),
                        },
                    )))],
                }),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(5)),
            }),
        );
        assert!(evaluate_condition(&expr, &variables));

        // For undefined variable, short-circuit prevents int() evaluation
        let expr_undefined = Expression::And(
            Box::new(Expression::Comparison {
                left: Box::new(Expression::Element(Box::new(SevenMarkElement::Variable(
                    sevenmark_parser::ast::VariableElement {
                        location: sevenmark_parser::ast::Location::synthesized(),
                        content: "undefined".to_string(),
                    },
                )))),
                operator: ComparisonOperator::NotEqual,
                right: Box::new(Expression::Null),
            }),
            Box::new(Expression::Comparison {
                left: Box::new(Expression::FunctionCall {
                    name: "int".to_string(),
                    arguments: vec![Expression::Element(Box::new(SevenMarkElement::Variable(
                        sevenmark_parser::ast::VariableElement {
                            location: sevenmark_parser::ast::Location::synthesized(),
                            content: "undefined".to_string(),
                        },
                    )))],
                }),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(5)),
            }),
        );
        // undefined != null is false, so right side not evaluated
        assert!(!evaluate_condition(&expr_undefined, &variables));
    }

    #[test]
    fn test_bool_comparison() {
        let variables = HashMap::new();

        // (5 > 3) == (10 > 8) → true == true → true
        let expr = Expression::Comparison {
            left: Box::new(Expression::Comparison {
                left: Box::new(Expression::NumberLiteral(5)),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(3)),
            }),
            operator: ComparisonOperator::Equal,
            right: Box::new(Expression::Comparison {
                left: Box::new(Expression::NumberLiteral(10)),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(8)),
            }),
        };
        assert!(evaluate_condition(&expr, &variables));

        // (5 > 3) == (10 < 8) → true == false → false
        let expr2 = Expression::Comparison {
            left: Box::new(Expression::Comparison {
                left: Box::new(Expression::NumberLiteral(5)),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(3)),
            }),
            operator: ComparisonOperator::Equal,
            right: Box::new(Expression::Comparison {
                left: Box::new(Expression::NumberLiteral(10)),
                operator: ComparisonOperator::LessThan,
                right: Box::new(Expression::NumberLiteral(8)),
            }),
        };
        assert!(!evaluate_condition(&expr2, &variables));
    }

    #[test]
    fn test_incomparable_types() {
        let variables = HashMap::new();

        // "abc" < 5 → false (비교 불가, 0으로 변환하지 않음)
        let expr = Expression::Comparison {
            left: Box::new(Expression::StringLiteral("abc".to_string())),
            operator: ComparisonOperator::LessThan,
            right: Box::new(Expression::NumberLiteral(5)),
        };
        assert!(!evaluate_condition(&expr, &variables));

        // "abc" > 5 → false (비교 불가)
        let expr2 = Expression::Comparison {
            left: Box::new(Expression::StringLiteral("abc".to_string())),
            operator: ComparisonOperator::GreaterThan,
            right: Box::new(Expression::NumberLiteral(5)),
        };
        assert!(!evaluate_condition(&expr2, &variables));

        // "10" > 5 → true (문자열이 숫자로 파싱 가능)
        let expr3 = Expression::Comparison {
            left: Box::new(Expression::StringLiteral("10".to_string())),
            operator: ComparisonOperator::GreaterThan,
            right: Box::new(Expression::NumberLiteral(5)),
        };
        assert!(evaluate_condition(&expr3, &variables));

        // null > 5 → false (null은 숫자 비교 불가)
        let expr4 = Expression::Comparison {
            left: Box::new(Expression::Null),
            operator: ComparisonOperator::GreaterThan,
            right: Box::new(Expression::NumberLiteral(5)),
        };
        assert!(!evaluate_condition(&expr4, &variables));
    }

    #[test]
    fn test_bool_literal() {
        let variables = HashMap::new();

        // true == true → true
        let expr = Expression::Comparison {
            left: Box::new(Expression::BoolLiteral(true)),
            operator: ComparisonOperator::Equal,
            right: Box::new(Expression::BoolLiteral(true)),
        };
        assert!(evaluate_condition(&expr, &variables));

        // (5 > 3) == true → true
        let expr2 = Expression::Comparison {
            left: Box::new(Expression::Comparison {
                left: Box::new(Expression::NumberLiteral(5)),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(Expression::NumberLiteral(3)),
            }),
            operator: ComparisonOperator::Equal,
            right: Box::new(Expression::BoolLiteral(true)),
        };
        assert!(evaluate_condition(&expr2, &variables));

        // false || true → true
        let expr3 = Expression::Or(
            Box::new(Expression::BoolLiteral(false)),
            Box::new(Expression::BoolLiteral(true)),
        );
        assert!(evaluate_condition(&expr3, &variables));

        // !false → true
        let expr4 = Expression::Not(Box::new(Expression::BoolLiteral(false)));
        assert!(evaluate_condition(&expr4, &variables));
    }
}
