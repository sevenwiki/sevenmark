use crate::ast::{
    AstNode, Expression, ListItem, ListItemChild, NodeKind, TableCell, TableCellChild, TableRow,
    TableRowChild,
};

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
            | NodeKind::If { children, .. } => {
                children.iter_mut().for_each(visitor);
            }

            // === Fold: content (two inner elements) ===
            NodeKind::Fold { content, .. } => {
                content.0.children.iter_mut().for_each(&mut *visitor);
                content.1.children.iter_mut().for_each(&mut *visitor);
            }

            // === Table: rows with cells ===
            NodeKind::Table { children, .. } => {
                for row_child in children {
                    traverse_table_row_child(row_child, visitor);
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item_child in children {
                    traverse_list_item_child(item_child, visitor);
                }
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
            | NodeKind::If { children, .. } => {
                f(children);
            }

            // === Fold: content (two inner elements) ===
            NodeKind::Fold { content, .. } => {
                f(&mut content.0.children);
                f(&mut content.1.children);
            }

            // === Table: rows contain cells ===
            NodeKind::Table { children, .. } => {
                for row_child in children {
                    for_each_table_row_child_vec(row_child, f);
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item_child in children {
                    for_each_list_item_child_vec(item_child, f);
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
            | NodeKind::If { children, .. } => {
                children.iter().for_each(visitor);
            }

            // === Fold: content (two inner elements) ===
            NodeKind::Fold { content, .. } => {
                content.0.children.iter().for_each(&mut *visitor);
                content.1.children.iter().for_each(&mut *visitor);
            }

            // === Table: rows with cells ===
            NodeKind::Table { children, .. } => {
                for row_child in children {
                    traverse_table_row_child_ref(row_child, visitor);
                }
            }

            // === List: items ===
            NodeKind::List { children, .. } => {
                for item_child in children {
                    traverse_list_item_child_ref(item_child, visitor);
                }
            }
        }
    }
}

// === Helper functions for nested structures ===

fn traverse_table_row_child<F>(row_child: &mut TableRowChild, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    match row_child {
        TableRowChild::Row(row) => traverse_table_row(row, visitor),
        TableRowChild::Conditional {
            condition,
            children,
            ..
        } => {
            for row in children {
                traverse_table_row(row, visitor);
            }
            traverse_expression(condition, visitor);
        }
    }
}

fn traverse_table_row<F>(row: &mut TableRow, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    for cell_child in &mut row.children {
        traverse_table_cell_child(cell_child, visitor);
    }
}

fn traverse_table_cell_child<F>(cell_child: &mut TableCellChild, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    match cell_child {
        TableCellChild::Cell(cell) => traverse_table_cell(cell, visitor),
        TableCellChild::Conditional {
            condition,
            children,
            ..
        } => {
            for cell in children {
                traverse_table_cell(cell, visitor);
            }
            traverse_expression(condition, visitor);
        }
    }
}

fn traverse_table_cell<F>(cell: &mut TableCell, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    cell.x.iter_mut().for_each(&mut *visitor);
    cell.y.iter_mut().for_each(&mut *visitor);
    cell.children.iter_mut().for_each(&mut *visitor);
}

fn traverse_list_item_child<F>(item_child: &mut ListItemChild, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    match item_child {
        ListItemChild::Item(item) => traverse_list_item(item, visitor),
        ListItemChild::Conditional {
            condition,
            children,
            ..
        } => {
            for item in children {
                traverse_list_item(item, visitor);
            }
            traverse_expression(condition, visitor);
        }
    }
}

fn traverse_list_item<F>(item: &mut ListItem, visitor: &mut F)
where
    F: FnMut(&mut AstNode),
{
    item.children.iter_mut().for_each(visitor);
}

// === for_each_children_vec helpers ===

fn for_each_table_row_child_vec<F>(row_child: &mut TableRowChild, f: &mut F)
where
    F: FnMut(&mut Vec<AstNode>),
{
    match row_child {
        TableRowChild::Row(row) => {
            for cell_child in &mut row.children {
                for_each_table_cell_child_vec(cell_child, f);
            }
        }
        TableRowChild::Conditional { children, .. } => {
            for row in children {
                for cell_child in &mut row.children {
                    for_each_table_cell_child_vec(cell_child, f);
                }
            }
        }
    }
}

fn for_each_table_cell_child_vec<F>(cell_child: &mut TableCellChild, f: &mut F)
where
    F: FnMut(&mut Vec<AstNode>),
{
    match cell_child {
        TableCellChild::Cell(cell) => f(&mut cell.children),
        TableCellChild::Conditional { children, .. } => {
            for cell in children {
                f(&mut cell.children);
            }
        }
    }
}

fn for_each_list_item_child_vec<F>(item_child: &mut ListItemChild, f: &mut F)
where
    F: FnMut(&mut Vec<AstNode>),
{
    match item_child {
        ListItemChild::Item(item) => f(&mut item.children),
        ListItemChild::Conditional { children, .. } => {
            for item in children {
                f(&mut item.children);
            }
        }
    }
}

// === Ref helpers ===

fn traverse_table_row_child_ref<F>(row_child: &TableRowChild, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    match row_child {
        TableRowChild::Row(row) => traverse_table_row_ref(row, visitor),
        TableRowChild::Conditional {
            condition,
            children,
            ..
        } => {
            for row in children {
                traverse_table_row_ref(row, visitor);
            }
            traverse_expression_ref(condition, visitor);
        }
    }
}

fn traverse_table_row_ref<F>(row: &TableRow, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    for cell_child in &row.children {
        traverse_table_cell_child_ref(cell_child, visitor);
    }
}

fn traverse_table_cell_child_ref<F>(cell_child: &TableCellChild, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    match cell_child {
        TableCellChild::Cell(cell) => traverse_table_cell_ref(cell, visitor),
        TableCellChild::Conditional {
            condition,
            children,
            ..
        } => {
            for cell in children {
                traverse_table_cell_ref(cell, visitor);
            }
            traverse_expression_ref(condition, visitor);
        }
    }
}

fn traverse_table_cell_ref<F>(cell: &TableCell, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    cell.x.iter().for_each(&mut *visitor);
    cell.y.iter().for_each(&mut *visitor);
    cell.children.iter().for_each(&mut *visitor);
}

fn traverse_list_item_child_ref<F>(item_child: &ListItemChild, visitor: &mut F)
where
    F: FnMut(&AstNode),
{
    match item_child {
        ListItemChild::Item(item) => traverse_list_item_ref(item, visitor),
        ListItemChild::Conditional {
            condition,
            children,
            ..
        } => {
            for item in children {
                traverse_list_item_ref(item, visitor);
            }
            traverse_expression_ref(condition, visitor);
        }
    }
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