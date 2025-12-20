use crate::ast::{Expression, ListContentItem, SevenMarkElement, TableCellItem, TableRowItem};

/// Trait for automatically traversing AST elements
pub trait Traversable {
    /// 각 자식 요소에 대해 visitor 호출 (mutable)
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut SevenMarkElement);

    /// 각 자식 요소에 대해 visitor 호출 (immutable - 읽기 전용 순회)
    fn traverse_children_ref<F>(&self, visitor: &mut F)
    where
        F: FnMut(&SevenMarkElement);

    /// 각 content Vec에 대해 f 호출 (Vec 구조 변경이 필요할 때 사용)
    fn for_each_content_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<SevenMarkElement>);
}

impl Traversable for SevenMarkElement {
    fn traverse_children<F>(&mut self, visitor: &mut F)
    where
        F: FnMut(&mut SevenMarkElement),
    {
        match self {
            // 자식이 없는 요소들
            SevenMarkElement::Text(_)
            | SevenMarkElement::Comment(_)
            | SevenMarkElement::Escape(_)
            | SevenMarkElement::Error(_)
            | SevenMarkElement::Age(_)
            | SevenMarkElement::Variable(_)
            | SevenMarkElement::TeXElement(_)
            | SevenMarkElement::Null
            | SevenMarkElement::FootNote
            | SevenMarkElement::TimeNow
            | SevenMarkElement::NewLine
            | SevenMarkElement::HLine => {
                // 자식 없음
            }

            // content 필드 하나만 있는 요소들
            SevenMarkElement::LiteralElement(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Header(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Category(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::Redirect(e) => e.content.iter_mut().for_each(visitor),
            SevenMarkElement::FootnoteElement(e) => e.content.iter_mut().for_each(visitor),

            // content + parameters 둘 다 있는 요소들
            SevenMarkElement::StyledElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::BlockQuoteElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::RubyElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::CodeElement(e) => {
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::Include(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::MediaElement(e) => {
                for child in &mut e.content {
                    visitor(child);
                }
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }

            // parameters만 있는 요소
            SevenMarkElement::DefineElement(e) => {
                for param in e.parameters.values_mut() {
                    for child in &mut param.value {
                        visitor(child);
                    }
                }
            }

            // TextStyle 계열
            SevenMarkElement::Bold(e)
            | SevenMarkElement::Italic(e)
            | SevenMarkElement::Strikethrough(e)
            | SevenMarkElement::Underline(e)
            | SevenMarkElement::Superscript(e)
            | SevenMarkElement::Subscript(e) => {
                e.content.iter_mut().for_each(visitor);
            }

            // 특수 중첩 구조들
            SevenMarkElement::TableElement(table) => {
                for row_item in &mut table.content {
                    match row_item {
                        TableRowItem::Row(row) => {
                            for cell_item in &mut row.inner_content {
                                match cell_item {
                                    TableCellItem::Cell(cell) => {
                                        for child in &mut cell.content {
                                            visitor(child);
                                        }
                                    }
                                    TableCellItem::Conditional {
                                        condition, cells, ..
                                    } => {
                                        for cell in cells {
                                            for child in &mut cell.content {
                                                visitor(child);
                                            }
                                        }
                                        traverse_expression(condition, visitor);
                                    }
                                }
                            }
                        }
                        TableRowItem::Conditional {
                            condition, rows, ..
                        } => {
                            for row in rows {
                                for cell_item in &mut row.inner_content {
                                    match cell_item {
                                        TableCellItem::Cell(cell) => {
                                            for child in &mut cell.content {
                                                visitor(child);
                                            }
                                        }
                                        TableCellItem::Conditional {
                                            condition: cell_cond,
                                            cells,
                                            ..
                                        } => {
                                            for cell in cells {
                                                for child in &mut cell.content {
                                                    visitor(child);
                                                }
                                            }
                                            traverse_expression(cell_cond, visitor);
                                        }
                                    }
                                }
                            }
                            traverse_expression(condition, visitor);
                        }
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &mut list.content {
                    match item {
                        ListContentItem::Item(list_item) => {
                            for child in &mut list_item.content {
                                visitor(child);
                            }
                        }
                        ListContentItem::Conditional {
                            condition, items, ..
                        } => {
                            for list_item in items {
                                for child in &mut list_item.content {
                                    visitor(child);
                                }
                            }
                            traverse_expression(condition, visitor);
                        }
                    }
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                for child in &mut fold.content.0.content {
                    visitor(child);
                }
                for child in &mut fold.content.1.content {
                    visitor(child);
                }
            }

            // IfElement - content와 condition 내 Element들 순회
            SevenMarkElement::IfElement(if_elem) => {
                for child in &mut if_elem.content {
                    visitor(child);
                }
                // condition 내 Expression::Element들도 순회
                traverse_expression(&mut if_elem.condition, visitor);
            }
        }
    }

    fn for_each_content_vec<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Vec<SevenMarkElement>),
    {
        match self {
            // 자식이 없는 요소들
            SevenMarkElement::Text(_)
            | SevenMarkElement::Comment(_)
            | SevenMarkElement::Escape(_)
            | SevenMarkElement::Error(_)
            | SevenMarkElement::Age(_)
            | SevenMarkElement::Variable(_)
            | SevenMarkElement::TeXElement(_)
            | SevenMarkElement::CodeElement(_)
            | SevenMarkElement::Null
            | SevenMarkElement::FootNote
            | SevenMarkElement::TimeNow
            | SevenMarkElement::NewLine
            | SevenMarkElement::HLine
            | SevenMarkElement::DefineElement(_) => {}

            // content Vec 하나만 있는 요소들
            SevenMarkElement::LiteralElement(e) => f(&mut e.content),
            SevenMarkElement::Header(e) => f(&mut e.content),
            SevenMarkElement::Category(e) => f(&mut e.content),
            SevenMarkElement::Redirect(e) => f(&mut e.content),
            SevenMarkElement::FootnoteElement(e) => f(&mut e.content),
            SevenMarkElement::StyledElement(e) => f(&mut e.content),
            SevenMarkElement::BlockQuoteElement(e) => f(&mut e.content),
            SevenMarkElement::RubyElement(e) => f(&mut e.content),
            SevenMarkElement::Include(e) => f(&mut e.content),
            SevenMarkElement::MediaElement(e) => f(&mut e.content),
            SevenMarkElement::IfElement(e) => f(&mut e.content),

            // TextStyle 계열
            SevenMarkElement::Bold(e)
            | SevenMarkElement::Italic(e)
            | SevenMarkElement::Strikethrough(e)
            | SevenMarkElement::Underline(e)
            | SevenMarkElement::Superscript(e)
            | SevenMarkElement::Subscript(e) => f(&mut e.content),

            // 특수 중첩 구조들
            // Note: for_each_content_vec은 Vec<SevenMarkElement>에만 적용됨
            // Table/List의 Conditional은 typed content (rows/cells/items)를 가지므로
            // 여기서는 일반 셀의 content만 처리
            SevenMarkElement::TableElement(table) => {
                for row_item in &mut table.content {
                    match row_item {
                        TableRowItem::Row(row) => {
                            for cell_item in &mut row.inner_content {
                                if let TableCellItem::Cell(cell) = cell_item {
                                    f(&mut cell.content);
                                }
                                // TableCellItem::Conditional의 cells는 Vec<TableInnerElement2>이므로 처리하지 않음
                            }
                        }
                        TableRowItem::Conditional { rows, .. } => {
                            for row in rows {
                                for cell_item in &mut row.inner_content {
                                    if let TableCellItem::Cell(cell) = cell_item {
                                        f(&mut cell.content);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &mut list.content {
                    match item {
                        ListContentItem::Item(list_item) => f(&mut list_item.content),
                        ListContentItem::Conditional { items, .. } => {
                            for list_item in items {
                                f(&mut list_item.content);
                            }
                        }
                    }
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                f(&mut fold.content.0.content);
                f(&mut fold.content.1.content);
            }
        }
    }

    fn traverse_children_ref<F>(&self, visitor: &mut F)
    where
        F: FnMut(&SevenMarkElement),
    {
        match self {
            // 자식이 없는 요소들
            SevenMarkElement::Text(_)
            | SevenMarkElement::Comment(_)
            | SevenMarkElement::Escape(_)
            | SevenMarkElement::Error(_)
            | SevenMarkElement::Age(_)
            | SevenMarkElement::Variable(_)
            | SevenMarkElement::TeXElement(_)
            | SevenMarkElement::Null
            | SevenMarkElement::FootNote
            | SevenMarkElement::TimeNow
            | SevenMarkElement::NewLine
            | SevenMarkElement::HLine => {
                // 자식 없음
            }

            // content 필드 하나만 있는 요소들
            SevenMarkElement::LiteralElement(e) => e.content.iter().for_each(visitor),
            SevenMarkElement::Header(e) => e.content.iter().for_each(visitor),
            SevenMarkElement::Category(e) => e.content.iter().for_each(visitor),
            SevenMarkElement::Redirect(e) => e.content.iter().for_each(visitor),
            SevenMarkElement::FootnoteElement(e) => e.content.iter().for_each(visitor),

            // content + parameters 둘 다 있는 요소들
            SevenMarkElement::StyledElement(e) => {
                for child in &e.content {
                    visitor(child);
                }
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::BlockQuoteElement(e) => {
                for child in &e.content {
                    visitor(child);
                }
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::RubyElement(e) => {
                for child in &e.content {
                    visitor(child);
                }
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::CodeElement(e) => {
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::Include(e) => {
                for child in &e.content {
                    visitor(child);
                }
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }
            SevenMarkElement::MediaElement(e) => {
                for child in &e.content {
                    visitor(child);
                }
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }

            // parameters만 있는 요소
            SevenMarkElement::DefineElement(e) => {
                for param in e.parameters.values() {
                    for child in &param.value {
                        visitor(child);
                    }
                }
            }

            // TextStyle 계열
            SevenMarkElement::Bold(e)
            | SevenMarkElement::Italic(e)
            | SevenMarkElement::Strikethrough(e)
            | SevenMarkElement::Underline(e)
            | SevenMarkElement::Superscript(e)
            | SevenMarkElement::Subscript(e) => {
                e.content.iter().for_each(visitor);
            }

            // 특수 중첩 구조들
            SevenMarkElement::TableElement(table) => {
                for row_item in &table.content {
                    match row_item {
                        TableRowItem::Row(row) => {
                            for cell_item in &row.inner_content {
                                match cell_item {
                                    TableCellItem::Cell(cell) => {
                                        for child in &cell.content {
                                            visitor(child);
                                        }
                                    }
                                    TableCellItem::Conditional {
                                        condition, cells, ..
                                    } => {
                                        for cell in cells {
                                            for child in &cell.content {
                                                visitor(child);
                                            }
                                        }
                                        traverse_expression_ref(condition, visitor);
                                    }
                                }
                            }
                        }
                        TableRowItem::Conditional {
                            condition, rows, ..
                        } => {
                            for row in rows {
                                for cell_item in &row.inner_content {
                                    match cell_item {
                                        TableCellItem::Cell(cell) => {
                                            for child in &cell.content {
                                                visitor(child);
                                            }
                                        }
                                        TableCellItem::Conditional {
                                            condition: cell_cond,
                                            cells,
                                            ..
                                        } => {
                                            for cell in cells {
                                                for child in &cell.content {
                                                    visitor(child);
                                                }
                                            }
                                            traverse_expression_ref(cell_cond, visitor);
                                        }
                                    }
                                }
                            }
                            traverse_expression_ref(condition, visitor);
                        }
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &list.content {
                    match item {
                        ListContentItem::Item(list_item) => {
                            for child in &list_item.content {
                                visitor(child);
                            }
                        }
                        ListContentItem::Conditional {
                            condition, items, ..
                        } => {
                            for list_item in items {
                                for child in &list_item.content {
                                    visitor(child);
                                }
                            }
                            traverse_expression_ref(condition, visitor);
                        }
                    }
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                for child in &fold.content.0.content {
                    visitor(child);
                }
                for child in &fold.content.1.content {
                    visitor(child);
                }
            }

            // IfElement - content와 condition 내 Element들 순회
            SevenMarkElement::IfElement(if_elem) => {
                for child in &if_elem.content {
                    visitor(child);
                }
                traverse_expression_ref(&if_elem.condition, visitor);
            }
        }
    }
}

/// Expression 내의 SevenMarkElement들을 순회하는 헬퍼 함수 (mutable)
fn traverse_expression<F>(expr: &mut Expression, visitor: &mut F)
where
    F: FnMut(&mut SevenMarkElement),
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
        | Expression::Null { .. } => {
            // 자식 없음
        }
    }
}

/// Expression 내의 SevenMarkElement들을 순회하는 헬퍼 함수 (immutable)
fn traverse_expression_ref<F>(expr: &Expression, visitor: &mut F)
where
    F: FnMut(&SevenMarkElement),
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
        | Expression::Null { .. } => {
            // 자식 없음
        }
    }
}
