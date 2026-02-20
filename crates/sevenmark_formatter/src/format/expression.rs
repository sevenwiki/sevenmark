use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{ComparisonOperatorKind, Expression};

use crate::format::element::{format_element, format_elements};

pub fn format_expr<'a>(a: &'a Arena<'a>, expr: &Expression) -> DocBuilder<'a, Arena<'a>> {
    match expr {
        Expression::Or { left, right, .. } => format_expr(a, left)
            .append(a.text(" || "))
            .append(format_expr(a, right)),

        Expression::And { left, right, .. } => format_expr(a, left)
            .append(a.text(" && "))
            .append(format_expr(a, right)),

        Expression::Not { inner, .. } => a.text("!").append(format_expr(a, inner)),

        Expression::Comparison {
            left,
            operator,
            right,
            ..
        } => {
            let op_str = match operator.kind {
                ComparisonOperatorKind::Equal => "==",
                ComparisonOperatorKind::NotEqual => "!=",
                ComparisonOperatorKind::GreaterThan => ">",
                ComparisonOperatorKind::LessThan => "<",
                ComparisonOperatorKind::GreaterEqual => ">=",
                ComparisonOperatorKind::LessEqual => "<=",
            };
            format_expr(a, left)
                .append(a.text(format!(" {} ", op_str)))
                .append(format_expr(a, right))
        }

        Expression::FunctionCall {
            name, arguments, ..
        } => {
            let args = a.intersperse(
                arguments.iter().map(|arg| format_expr(a, arg)),
                a.text(", "),
            );
            a.text(format!("{}(", name))
                .append(args)
                .append(a.text(")"))
        }

        Expression::StringLiteral { value, .. } => a
            .text("\"")
            .append(format_elements(a, value))
            .append(a.text("\"")),

        Expression::NumberLiteral { value, .. } => a.text(value.to_string()),

        Expression::BoolLiteral { value, .. } => a.text(value.to_string()),

        Expression::Null { .. } => a.text("null"),

        Expression::Group { inner, .. } => a
            .text("(")
            .append(format_expr(a, inner))
            .append(a.text(")")),

        Expression::Element(elem) => format_element(a, elem),
    }
}
