use super::*;

fn rs(span: &mut Span, f: &impl Fn(usize) -> usize) {
    span.start = f(span.start);
    span.end = f(span.end);
}

fn remap_params(params: &mut Parameters, f: &impl Fn(usize) -> usize) {
    for param in params.values_mut() {
        rs(&mut param.span, f);
        remap_elements(&mut param.value, f);
    }
}

pub fn remap_elements(elements: &mut Vec<Element>, f: &impl Fn(usize) -> usize) {
    for element in elements {
        remap_element(element, f);
    }
}

pub fn remap_element(element: &mut Element, f: &impl Fn(usize) -> usize) {
    match element {
        Element::Text(e) => rs(&mut e.span, f),
        Element::Comment(e) => rs(&mut e.span, f),
        Element::Escape(e) => rs(&mut e.span, f),
        Element::Error(e) => rs(&mut e.span, f),
        Element::Null(e) => rs(&mut e.span, f),
        Element::FootnoteRef(e) => rs(&mut e.span, f),
        Element::TimeNow(e) => rs(&mut e.span, f),
        Element::Date(e) => rs(&mut e.span, f),
        Element::DateTime(e) => rs(&mut e.span, f),
        Element::Dday(e) => rs(&mut e.span, f),
        Element::PageCount(e) => rs(&mut e.span, f),
        Element::Age(e) => rs(&mut e.span, f),
        Element::Variable(e) => rs(&mut e.span, f),
        Element::Anchor(e) => rs(&mut e.span, f),
        Element::Toc(e) => rs(&mut e.span, f),
        Element::Mention(e) => rs(&mut e.span, f),
        Element::SoftBreak(e) => rs(&mut e.span, f),
        Element::HardBreak(e) => rs(&mut e.span, f),
        Element::Clear(e) => rs(&mut e.span, f),
        Element::HLine(e) => rs(&mut e.span, f),

        Element::Bold(e)
        | Element::Italic(e)
        | Element::Strikethrough(e)
        | Element::Underline(e)
        | Element::Superscript(e)
        | Element::Subscript(e) => {
            rs(&mut e.span, f);
            remap_elements(&mut e.children, f);
        }

        Element::Header(e) => {
            rs(&mut e.span, f);
            remap_elements(&mut e.children, f);
        }

        Element::Literal(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_elements(&mut e.children, f);
        }
        Element::Define(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
        }
        Element::Styled(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::BlockQuote(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::Ruby(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::Footnote(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::Code(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
        }
        Element::TeX(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
        }
        Element::Css(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
        }
        Element::Include(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::Category(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_elements(&mut e.children, f);
        }
        Element::Redirect(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::Media(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            remap_elements(&mut e.children, f);
        }
        Element::ExternalMedia(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
        }
        Element::If(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_elements(&mut e.children, f);
        }

        Element::Fold(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            rs(&mut e.summary.span, f);
            rs(&mut e.summary.open_span, f);
            rs(&mut e.summary.close_span, f);
            remap_params(&mut e.summary.parameters, f);
            remap_elements(&mut e.summary.children, f);
            rs(&mut e.details.span, f);
            rs(&mut e.details.open_span, f);
            rs(&mut e.details.close_span, f);
            remap_params(&mut e.details.parameters, f);
            remap_elements(&mut e.details.children, f);
        }

        Element::List(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            for item in &mut e.children {
                match item {
                    ListContentItem::Item(li) => remap_list_item(li, f),
                    ListContentItem::Conditional(cond) => {
                        rs(&mut cond.span, f);
                        rs(&mut cond.open_span, f);
                        rs(&mut cond.close_span, f);
                        for li in &mut cond.items {
                            remap_list_item(li, f);
                        }
                    }
                }
            }
        }

        Element::Table(e) => {
            rs(&mut e.span, f);
            rs(&mut e.open_span, f);
            rs(&mut e.close_span, f);
            remap_params(&mut e.parameters, f);
            for row_item in &mut e.children {
                match row_item {
                    TableRowItem::Row(row) => remap_table_row(row, f),
                    TableRowItem::Conditional(cond) => {
                        rs(&mut cond.span, f);
                        rs(&mut cond.open_span, f);
                        rs(&mut cond.close_span, f);
                        for row in &mut cond.rows {
                            remap_table_row(row, f);
                        }
                    }
                }
            }
        }
    }
}

fn remap_list_item(li: &mut ListItemElement, f: &impl Fn(usize) -> usize) {
    rs(&mut li.span, f);
    rs(&mut li.open_span, f);
    rs(&mut li.close_span, f);
    remap_params(&mut li.parameters, f);
    remap_elements(&mut li.children, f);
}

fn remap_table_row(row: &mut TableRowElement, f: &impl Fn(usize) -> usize) {
    rs(&mut row.span, f);
    rs(&mut row.open_span, f);
    rs(&mut row.close_span, f);
    remap_params(&mut row.parameters, f);
    for cell_item in &mut row.children {
        match cell_item {
            TableCellItem::Cell(cell) => remap_table_cell(cell, f),
            TableCellItem::Conditional(cond) => {
                rs(&mut cond.span, f);
                rs(&mut cond.open_span, f);
                rs(&mut cond.close_span, f);
                for cell in &mut cond.cells {
                    remap_table_cell(cell, f);
                }
            }
        }
    }
}

fn remap_table_cell(cell: &mut TableCellElement, f: &impl Fn(usize) -> usize) {
    rs(&mut cell.span, f);
    rs(&mut cell.open_span, f);
    rs(&mut cell.close_span, f);
    remap_params(&mut cell.parameters, f);
    remap_elements(&mut cell.x, f);
    remap_elements(&mut cell.y, f);
    remap_elements(&mut cell.children, f);
}
