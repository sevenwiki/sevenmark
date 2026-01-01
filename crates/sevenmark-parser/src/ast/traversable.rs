use crate::ast::{AstNode, Expression, ListItem, NodeKind, TableCell, TableRow};

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
            NodeKind::Literal { children, .. }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. } => {
                children.iter_mut().for_each(visitor);
            }

            // === Fold: title + children ===
            NodeKind::Fold {
                title, children, ..
            } => {
                title.iter_mut().for_each(visitor);
                children.iter_mut().for_each(visitor);
            }

            // === Ruby: base + text ===
            NodeKind::Ruby { base, text, .. } => {
                base.iter_mut().for_each(visitor);
                text.iter_mut().for_each(visitor);
            }

            // === Table: rows with cells ===
            NodeKind::Table { children, .. } => {
                for row in children {
                    traverse_table_row(row, visitor);
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item in children {
                    traverse_list_item(item, visitor);
                }
            }

            // === Conditional groups ===
            NodeKind::ConditionalTableRows {
                condition,
                children,
            } => {
                for row in children {
                    traverse_table_row(row, visitor);
                }
                traverse_expression(condition, visitor);
            }
            NodeKind::ConditionalTableCells {
                condition,
                children,
            } => {
                for cell in children {
                    traverse_table_cell(cell, visitor);
                }
                traverse_expression(condition, visitor);
            }
            NodeKind::ConditionalListItems {
                condition,
                children,
            } => {
                for item in children {
                    traverse_list_item(item, visitor);
                }
                traverse_expression(condition, visitor);
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
            NodeKind::Literal { children, .. }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. } => {
                f(children);
            }

            // === Fold: title + children ===
            NodeKind::Fold {
                title, children, ..
            } => {
                f(title);
                f(children);
            }

            // === Ruby: base + text ===
            NodeKind::Ruby { base, text, .. } => {
                f(base);
                f(text);
            }

            // === Table: rows contain cells ===
            NodeKind::Table { children, .. } => {
                for row in children {
                    for cell in &mut row.children {
                        f(&mut cell.children);
                    }
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item in children {
                    f(&mut item.children);
                }
            }

            // === Conditional groups ===
            NodeKind::ConditionalTableRows { children, .. } => {
                for row in children {
                    for cell in &mut row.children {
                        f(&mut cell.children);
                    }
                }
            }
            NodeKind::ConditionalTableCells { children, .. } => {
                for cell in children {
                    f(&mut cell.children);
                }
            }
            NodeKind::ConditionalListItems { children, .. } => {
                for item in children {
                    f(&mut item.children);
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
            NodeKind::Literal { children, .. }
            | NodeKind::Styled { children, .. }
            | NodeKind::BlockQuote { children, .. }
            | NodeKind::Footnote { children, .. }
            | NodeKind::Include { children, .. }
            | NodeKind::Category { children }
            | NodeKind::Redirect { children, .. }
            | NodeKind::Media { children, .. }
            | NodeKind::Bold { children }
            | NodeKind::Italic { children }
            | NodeKind::Strikethrough { children }
            | NodeKind::Underline { children }
            | NodeKind::Superscript { children }
            | NodeKind::Subscript { children }
            | NodeKind::Header { children, .. }
            | NodeKind::If { children, .. } => {
                children.iter().for_each(visitor);
            }

            // === Fold: title + children ===
            NodeKind::Fold {
                title, children, ..
            } => {
                title.iter().for_each(visitor);
                children.iter().for_each(visitor);
            }

            // === Ruby: base + text ===
            NodeKind::Ruby { base, text, .. } => {
                base.iter().for_each(visitor);
                text.iter().for_each(visitor);
            }

            // === Table: rows with cells ===
            NodeKind::Table { children, .. } => {
                for row in children {
                    traverse_table_row_ref(row, visitor);
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item in children {
                    traverse_list_item_ref(item, visitor);
                }
            }

            // === Conditional groups ===
            NodeKind::ConditionalTableRows {
                condition,
                children,
            } => {
                for row in children {
                    traverse_table_row_ref(row, visitor);
                }
                traverse_expression_ref(condition, visitor);
            }
            NodeKind::ConditionalTableCells {
                condition,
                children,
            } => {
                for cell in children {
                    traverse_table_cell_ref(cell, visitor);
                }
                traverse_expression_ref(condition, visitor);
            }
            NodeKind::ConditionalListItems {
                condition,
                children,
            } => {
                for item in children {
                    traverse_list_item_ref(item, visitor);
                }
                traverse_expression_ref(condition, visitor);
            }
        }
    }
}

// === Helper functions for nested structures ===

fn traverse_table_row<F>(row: &mut TableRow, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    for cell in &mut row.children {
        traverse_table_cell(cell, visitor);
    }
}

fn traverse_table_cell<F>(cell: &mut TableCell, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    cell.x.iter_mut().for_each(visitor);
    cell.y.iter_mut().for_each(visitor);
    cell.children.iter_mut().for_each(visitor);
}

fn traverse_list_item<F>(item: &mut ListItem, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    item.children.iter_mut().for_each(visitor);
}

fn traverse_table_row_ref<F>(row: &TableRow, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    for cell in &row.children {
        traverse_table_cell_ref(cell, visitor);
    }
}

fn traverse_table_cell_ref<F>(cell: &TableCell, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    cell.x.iter().for_each(visitor);
    cell.y.iter().for_each(visitor);
    cell.children.iter().for_each(visitor);
}

fn traverse_list_item_ref<F>(item: &ListItem, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    item.children.iter().for_each(visitor);
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