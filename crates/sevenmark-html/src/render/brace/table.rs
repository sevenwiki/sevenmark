//! Table rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{TableCellItem, TableElement, TableRowItem};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &TableElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let style = utils::build_style(&e.parameters);
    let content = html! {
        div class=(classes::TABLE_WRAPPER) {
            table class=(classes::TABLE) style=[style] {
                tbody {
                    @for row_item in &e.content {
                        @match row_item {
                            TableRowItem::Row(row) => {
                                @let row_style = utils::build_style(&row.parameters);
                                tr style=[row_style] { (render_cells(&row.content, ctx)) }
                            }
                            TableRowItem::Conditional { rows, .. } => {
                                @for row in rows {
                                    @let row_style = utils::build_style(&row.parameters);
                                    tr style=[row_style] { (render_cells(&row.content, ctx)) }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    ctx.exit_suppress_soft_breaks();
    content
}

fn render_cells(cells: &[TableCellItem], ctx: &mut RenderContext) -> Markup {
    html! {
        @for cell_item in cells {
            @match cell_item {
                TableCellItem::Cell(cell) => {
                    @let colspan = utils::extract_text(&cell.x).parse::<usize>().ok().filter(|&n| n > 1);
                    @let rowspan = utils::extract_text(&cell.y).parse::<usize>().ok().filter(|&n| n > 1);
                    @let style = utils::build_style(&cell.parameters);
                    td colspan=[colspan] rowspan=[rowspan] style=[style] {
                        (render_elements(&cell.content, ctx))
                    }
                }
                TableCellItem::Conditional { cells, .. } => {
                    @for cell in cells {
                        @let colspan = utils::extract_text(&cell.x).parse::<usize>().ok().filter(|&n| n > 1);
                        @let rowspan = utils::extract_text(&cell.y).parse::<usize>().ok().filter(|&n| n > 1);
                        @let style = utils::build_style(&cell.parameters);
                        td colspan=[colspan] rowspan=[rowspan] style=[style] {
                            (render_elements(&cell.content, ctx))
                        }
                    }
                }
            }
        }
    }
}
