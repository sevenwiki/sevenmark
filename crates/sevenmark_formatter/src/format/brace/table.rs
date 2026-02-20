use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{
    ConditionalTableCells, ConditionalTableRows, TableCellElement, TableCellItem, TableElement,
    TableRowElement, TableRowItem,
};

use crate::format::element::format_elements;
use crate::format::expression::format_expr;
use crate::format::params::{format_params_block, format_params_block_tight};

const INDENT: isize = 2;

pub fn format_table<'a>(a: &'a Arena<'a>, e: &TableElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block(a, &e.parameters);
    let rows = a.intersperse(
        e.children.iter().map(|r| format_row_item(a, r)),
        a.hardline(),
    );
    a.text("{{{#table")
        .append(params)
        .append(a.hardline().append(rows).nest(INDENT))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_row_item<'a>(a: &'a Arena<'a>, item: &TableRowItem) -> DocBuilder<'a, Arena<'a>> {
    match item {
        TableRowItem::Row(row) => format_row(a, row),
        TableRowItem::Conditional(cond) => format_conditional_rows(a, cond),
    }
}

fn format_row<'a>(a: &'a Arena<'a>, row: &TableRowElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &row.parameters);
    // cells: 한 줄에 들어가면 공백 구분, 넘으면 줄바꿈
    let cells = a.intersperse(
        row.children.iter().map(|c| format_cell_item(a, c)),
        a.line(),
    );
    a.text("[[")
        .append(params)
        .append(a.hardline().append(cells.group()).nest(INDENT))
        .append(a.hardline())
        .append(a.text("]]"))
}

fn format_cell_item<'a>(a: &'a Arena<'a>, item: &TableCellItem) -> DocBuilder<'a, Arena<'a>> {
    match item {
        TableCellItem::Cell(cell) => format_cell(a, cell),
        TableCellItem::Conditional(cond) => format_conditional_cells(a, cond),
    }
}

fn format_cell<'a>(a: &'a Arena<'a>, cell: &TableCellElement) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &cell.parameters);
    let has_params = !cell.parameters.is_empty();
    a.text("[[")
        .append(params)
        .append(if cell.children.is_empty() {
            a.nil()
        } else if has_params {
            // params 뒤에 || 있으니 공백 후 content
            a.text(" ").append(format_elements(a, &cell.children))
        } else {
            format_elements(a, &cell.children)
        })
        .append(a.text("]]"))
}

fn format_conditional_rows<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalTableRows,
) -> DocBuilder<'a, Arena<'a>> {
    let rows = a.intersperse(cond.rows.iter().map(|r| format_row(a, r)), a.hardline());
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition))
        .append(a.text(" ::"))
        .append(a.hardline().append(rows).nest(INDENT))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_conditional_cells<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalTableCells,
) -> DocBuilder<'a, Arena<'a>> {
    let cells = a.intersperse(cond.cells.iter().map(|c| format_cell(a, c)), a.line());
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition))
        .append(a.text(" ::"))
        .append(a.line().append(cells).nest(INDENT).group())
        .append(a.hardline())
        .append(a.text("}}}"))
}
