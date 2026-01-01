use crate::ast::{AstNode, Expression, NodeKind};

/// Trait for automatically traversing AST elements
pub trait Traversable {
    /// 각 자식 요소에 대해 visitor 호출 (mutable)
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut AstNode);

    /// 각 자식 요소에 대해 visitor 호출 (immutable - 읽기 전용 순회)
    fn traverse_children_ref<F>(&self, visitor: &mut F)
    where
        F: FnMut(&AstNode);

    /// 각 children Vec에 대해 f 호출 (Vec 구조 변경이 필요할 때 사용)
    fn for_each_children_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<AstNode>);
}

impl Traversable for AstNode {
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut AstNode),
    {
        match &mut self.kind {
            // === Leaf nodes (자식 없음) ===
            NodeKind::Text { .. }
            | NodeKind::Comment { .. }
            | NodeKind::Escape { .. }
            | NodeKind::Error { .. }
            | NodeKind::Code { .. }
            | NodeKind::TeX { .. }
            | NodeKind::Define { .. }
            | NodeKind::Null
            | NodeKind::FootnoteRef
            | NodeKind::TimeNow
            | NodeKind::Age { .. }
            | NodeKind::Variable { .. }
            | NodeKind::Mention { .. }
            | NodeKind::SoftBreak
            | NodeKind::HardBreak
            | NodeKind::HLine => {
                // 자식 없음
            }

            // === children 필드만 있는 노드들 ===
            NodeKind::Literal { children }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Ruby { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. }
            | NodeKind::Table { children, .. }
            | NodeKind::TableRow { children, .. }
            | NodeKind::List { children, .. }
            | NodeKind::ListItem { children, .. }
            | NodeKind::FoldInner { children, .. } => {
                children.iter_mut().for_each(visitor);
            }

            // === Conditional variants (Expression + children) ===
            NodeKind::ConditionalTableRows {
                condition,
                children,
            }
            | NodeKind::ConditionalTableCells {
                condition,
                children,
            }
            | NodeKind::ConditionalListItems {
                condition,
                children,
            } => {
                children.iter_mut().for_each(&mut *visitor);
                traverse_expression(condition, visitor);
            }

            // === TableCell: x, y, children ===
            NodeKind::TableCell { x, y, children, .. } => {
                x.iter_mut().for_each(&mut *visitor);
                y.iter_mut().for_each(&mut *visitor);
                children.iter_mut().for_each(&mut *visitor);
            }

            // === Fold: content 튜플 ===
            NodeKind::Fold { children, .. } => {
                visitor(&mut children.0);
                visitor(&mut children.1);
            }
        }
    }

    fn for_each_children_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<AstNode>),
    {
        match &mut self.kind {
            // === Leaf nodes ===
            NodeKind::Text { .. }
            | NodeKind::Comment { .. }
            | NodeKind::Escape { .. }
            | NodeKind::Error { .. }
            | NodeKind::Code { .. }
            | NodeKind::TeX { .. }
            | NodeKind::Define { .. }
            | NodeKind::Null
            | NodeKind::FootnoteRef
            | NodeKind::TimeNow
            | NodeKind::Age { .. }
            | NodeKind::Variable { .. }
            | NodeKind::Mention { .. }
            | NodeKind::SoftBreak
            | NodeKind::HardBreak
            | NodeKind::HLine => {}

            // === children 필드만 있는 노드들 ===
            NodeKind::Literal { children }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Ruby { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. }
            | NodeKind::Table { children, .. }
            | NodeKind::TableRow { children, .. }
            | NodeKind::List { children, .. }
            | NodeKind::ListItem { children, .. }
            | NodeKind::FoldInner { children, .. }
            | NodeKind::ConditionalTableRows { children, .. }
            | NodeKind::ConditionalTableCells { children, .. }
            | NodeKind::ConditionalListItems { children, .. } => {
                f(children);
            }

            // === TableCell: x, y, children ===
            NodeKind::TableCell { x, y, children, .. } => {
                f(x);
                f(y);
                f(children);
            }

            // === Fold: content 튜플 (FoldInner의 children에 접근) ===
            NodeKind::Fold { children, .. } => {
                if let NodeKind::FoldInner { children, .. } = &mut children.0.kind {
                    f(children);
                }
                if let NodeKind::FoldInner { children, .. } = &mut children.1.kind {
                    f(children);
                }
            }
        }
    }

    fn traverse_children_ref<F>(&self, visitor: &mut F)
    where
        F: FnMut(&AstNode),
    {
        match &self.kind {
            // === Leaf nodes ===
            NodeKind::Text { .. }
            | NodeKind::Comment { .. }
            | NodeKind::Escape { .. }
            | NodeKind::Error { .. }
            | NodeKind::Code { .. }
            | NodeKind::TeX { .. }
            | NodeKind::Define { .. }
            | NodeKind::Null
            | NodeKind::FootnoteRef
            | NodeKind::TimeNow
            | NodeKind::Age { .. }
            | NodeKind::Variable { .. }
            | NodeKind::Mention { .. }
            | NodeKind::SoftBreak
            | NodeKind::HardBreak
            | NodeKind::HLine => {}

            // === children 필드만 있는 노드들 ===
            NodeKind::Literal { children }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Ruby { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. }
            | NodeKind::Table { children, .. }
            | NodeKind::TableRow { children, .. }
            | NodeKind::List { children, .. }
            | NodeKind::ListItem { children, .. }
            | NodeKind::FoldInner { children, .. } => {
                children.iter().for_each(visitor);
            }

            // === Conditional variants (Expression + children) ===
            NodeKind::ConditionalTableRows {
                condition,
                children,
            }
            | NodeKind::ConditionalTableCells {
                condition,
                children,
            }
            | NodeKind::ConditionalListItems {
                condition,
                children,
            } => {
                children.iter().for_each(&mut *visitor);
                traverse_expression_ref(condition, visitor);
            }

            // === TableCell: x, y, children ===
            NodeKind::TableCell { x, y, children, .. } => {
                x.iter().for_each(&mut *visitor);
                y.iter().for_each(&mut *visitor);
                children.iter().for_each(&mut *visitor);
            }

            // === Fold: content 튜플 ===
            NodeKind::Fold { children, .. } => {
                visitor(&children.0);
                visitor(&children.1);
            }
        }
    }
}

// === Expression traversal ===

/// Expression 내의 AstNode들을 순회하는 헬퍼 함수 (mutable)
fn traverse_expression<F>(expr: &mut Expression, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    match expr {
        Expression::Or { left, right, .. } | Expression::And { left, right, .. } => {
            traverse_expression(left, visitor);
            traverse_expression(right, visitor);
        }
        Expression::Not { inner, .. } | Expression::Group { inner, .. } => {
            traverse_expression(inner, visitor);
        }
        Expression::Comparison { left, right, .. } => {
            traverse_expression(left, visitor);
            traverse_expression(right, visitor);
        }
        Expression::FunctionCall { arguments, .. } => {
            for arg in arguments {
                traverse_expression(arg, visitor);
            }
        }
        Expression::Element(elem) => {
            visitor(elem);
        }
        Expression::StringLiteral { .. }
        | Expression::NumberLiteral { .. }
        | Expression::BoolLiteral { .. }
        | Expression::Null { .. } => {}
    }
}

/// Expression 내의 AstNode들을 순회하는 헬퍼 함수 (immutable)
fn traverse_expression_ref<F>(expr: &Expression, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    match expr {
        Expression::Or { left, right, .. } | Expression::And { left, right, .. } => {
            traverse_expression_ref(left, visitor);
            traverse_expression_ref(right, visitor);
        }
        Expression::Not { inner, .. } | Expression::Group { inner, .. } => {
            traverse_expression_ref(inner, visitor);
        }
        Expression::Comparison { left, right, .. } => {
            traverse_expression_ref(left, visitor);
            traverse_expression_ref(right, visitor);
        }
        Expression::FunctionCall { arguments, .. } => {
            for arg in arguments {
                traverse_expression_ref(arg, visitor);
            }
        }
        Expression::Element(elem) => {
            visitor(elem);
        }
        Expression::StringLiteral { .. }
        | Expression::NumberLiteral { .. }
        | Expression::BoolLiteral { .. }
        | Expression::Null { .. } => {}
    }
}
