use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::{
    ConditionalTableCells, ConditionalTableRows, TableCellElement, TableCellItem, TableElement,
    TableRowElement, TableRowItem,
};

use crate::FormatConfig;
use crate::format::element::format_elements;
use crate::format::expression::format_expr;
use crate::format::params::{format_params_block, format_params_block_tight};

pub fn format_table<'a>(
    a: &'a Arena<'a>,
    e: &TableElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let params = format_params_block(a, &e.parameters, config);
    let rows = a.intersperse(
        e.children.iter().map(|r| format_row_item(a, r, config)),
        a.hardline(),
    );
    a.text("{{{#table")
        .append(params)
        .append(a.hardline().append(rows).nest(indent))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_row_item<'a>(
    a: &'a Arena<'a>,
    item: &TableRowItem,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    match item {
        TableRowItem::Row(row) => format_row(a, row, config),
        TableRowItem::Conditional(cond) => format_conditional_rows(a, cond, config),
    }
}

fn format_row<'a>(
    a: &'a Arena<'a>,
    row: &TableRowElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let params = format_params_block_tight(a, &row.parameters, config);
    let cells = a.intersperse(
        row.children.iter().map(|c| format_cell_item(a, c, config)),
        a.line(),
    );
    a.text("[[")
        .append(params)
        .append(a.hardline().append(cells.group()).nest(indent))
        .append(a.hardline())
        .append(a.text("]]"))
}

fn format_cell_item<'a>(
    a: &'a Arena<'a>,
    item: &TableCellItem,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    match item {
        TableCellItem::Cell(cell) => format_cell(a, cell, config),
        TableCellItem::Conditional(cond) => format_conditional_cells(a, cond, config),
    }
}

fn format_cell<'a>(
    a: &'a Arena<'a>,
    cell: &TableCellElement,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let params = format_params_block_tight(a, &cell.parameters, config);
    let has_params = !cell.parameters.is_empty();
    let content = if cell.children.is_empty() {
        a.nil()
    } else if has_params {
        a.text(" ")
            .append(format_elements(a, &cell.children, config))
    } else {
        format_elements(a, &cell.children, config)
    };

    let closing = if needs_line_break_before_cell_close(&cell.children) {
        a.hardline().append(a.text("]]"))
    } else {
        a.text("]]")
    };

    a.text("[[").append(params).append(content).append(closing)
}

fn format_conditional_rows<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalTableRows,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let rows = a.intersperse(
        cond.rows.iter().map(|r| format_row(a, r, config)),
        a.hardline(),
    );
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition, config))
        .append(a.text(" ::"))
        .append(a.hardline().append(rows).nest(indent))
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn format_conditional_cells<'a>(
    a: &'a Arena<'a>,
    cond: &ConditionalTableCells,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    let indent = config.indent as isize;
    let cells = a.intersperse(
        cond.cells.iter().map(|c| format_cell(a, c, config)),
        a.line(),
    );
    a.text("{{{#if ")
        .append(format_expr(a, &cond.condition, config))
        .append(a.text(" ::"))
        .append(a.line().append(cells).nest(indent).group())
        .append(a.hardline())
        .append(a.text("}}}"))
}

fn needs_line_break_before_cell_close(children: &[sevenmark_ast::Element]) -> bool {
    let last_semantic = children
        .iter()
        .rev()
        .find(|el| !is_ignorable_trailing_text(el));

    matches!(
        last_semantic,
        Some(
            sevenmark_ast::Element::Table(_)
                | sevenmark_ast::Element::List(_)
                | sevenmark_ast::Element::Fold(_)
                | sevenmark_ast::Element::Code(_)
                | sevenmark_ast::Element::TeX(_)
                | sevenmark_ast::Element::BlockQuote(_)
                | sevenmark_ast::Element::Literal(_)
                | sevenmark_ast::Element::Styled(_)
                | sevenmark_ast::Element::Include(_)
                | sevenmark_ast::Element::Footnote(_)
                | sevenmark_ast::Element::If(_)
        )
    )
}

fn is_ignorable_trailing_text(el: &sevenmark_ast::Element) -> bool {
    match el {
        sevenmark_ast::Element::Text(t) => t.value.chars().all(|c| matches!(c, ' ' | '\t' | '\r')),
        sevenmark_ast::Element::SoftBreak(_) => true,
        _ => false,
    }
}
