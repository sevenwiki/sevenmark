//! Table element renderer

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::{TableCellItem, TableElement, TableInnerElement1, TableRowItem};

/// Render table element
pub fn render_table(elem: &TableElement, ctx: &mut RenderContext) -> Markup {
    html! {
        table {
            tbody {
                @for row_item in &elem.content {
                    (render_row_item(row_item, ctx))
                }
            }
        }
    }
}

fn render_row_item(row_item: &TableRowItem, ctx: &mut RenderContext) -> Markup {
    match row_item {
        TableRowItem::Row(row) => render_row(row, ctx),
        TableRowItem::Conditional { rows, .. } => {
            // Conditional rows should be processed in transform, but render them if present
            html! {
                @for row in rows {
                    (render_row(row, ctx))
                }
            }
        }
    }
}

fn render_row(row: &TableInnerElement1, ctx: &mut RenderContext) -> Markup {
    html! {
        tr {
            @for cell_item in &row.inner_content {
                (render_cell_item(cell_item, ctx))
            }
        }
    }
}

fn render_cell_item(cell_item: &TableCellItem, ctx: &mut RenderContext) -> Markup {
    match cell_item {
        TableCellItem::Cell(cell) => {
            let colspan = if cell.x.is_empty() {
                None
            } else {
                Some(cell.x.len().to_string())
            };
            let rowspan = if cell.y.is_empty() {
                None
            } else {
                Some(cell.y.len().to_string())
            };

            html! {
                td colspan=[colspan] rowspan=[rowspan] {
                    (render_elements(&cell.content, ctx))
                }
            }
        }
        TableCellItem::Conditional { cells, .. } => {
            // Conditional cells should be processed in transform
            html! {
                @for cell in cells {
                    td {
                        (render_elements(&cell.content, ctx))
                    }
                }
            }
        }
    }
}
