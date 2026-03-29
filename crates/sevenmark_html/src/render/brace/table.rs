//! Table rendering

use maud::{Markup, html};
use sevenmark_ast::{Parameters, Span, TableCellItem, TableRowElement, TableRowItem};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(
    span: &Span,
    parameters: &Parameters,
    children: &[TableRowItem],
    ctx: &mut RenderContext,
) -> Markup {
    ctx.enter_suppress_soft_breaks();

    let style = utils::build_style(parameters);
    let class = utils::merge_class(classes::TABLE, parameters);
    let dark_style = utils::build_dark_style(parameters);
    let caption = utils::get_param(parameters, "caption");
    let sortable = parameters.contains_key("sortable");

    // Partition rows into head and body.
    // A row is a head row if it has the `#head` flag parameter.
    let mut head_rows: Vec<&TableRowElement> = Vec::new();
    let mut body_items: Vec<&TableRowItem> = Vec::new();

    for item in children {
        match item {
            TableRowItem::Row(row) => {
                if row.parameters.contains_key("head") {
                    head_rows.push(row);
                } else {
                    body_items.push(item);
                }
            }
            TableRowItem::Conditional(_) => {
                body_items.push(item);
            }
        }
    }

    let content = html! {
        div
            class=(classes::TABLE_WRAPPER)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        {
            table
                class=(class)
                style=[style]
                data-dark-style=[dark_style]
                data-sortable=[sortable.then_some("true")]
            {
                @if let Some(cap) = caption {
                    caption { (cap) }
                }
                @if !head_rows.is_empty() {
                    thead {
                        @for row in &head_rows {
                            @let row_style = utils::build_style(&row.parameters);
                            @let row_class = utils::param_class(&row.parameters);
                            @let row_dark_style = utils::build_dark_style(&row.parameters);
                            tr class=[row_class] style=[row_style] data-dark-style=[row_dark_style] {
                                (render_cells(&row.children, ctx, true))
                            }
                        }
                    }
                }
                tbody {
                    @for row_item in &body_items {
                        @match row_item {
                            TableRowItem::Row(row) => {
                                @let row_style = utils::build_style(&row.parameters);
                                @let row_class = utils::param_class(&row.parameters);
                                @let row_dark_style = utils::build_dark_style(&row.parameters);
                                tr class=[row_class] style=[row_style] data-dark-style=[row_dark_style] {
                                    (render_cells(&row.children, ctx, false))
                                }
                            }
                            TableRowItem::Conditional(cond) => {
                                @for row in &cond.rows {
                                    @let row_style = utils::build_style(&row.parameters);
                                    @let row_class = utils::param_class(&row.parameters);
                                    @let row_dark_style = utils::build_dark_style(&row.parameters);
                                    tr class=[row_class] style=[row_style] data-dark-style=[row_dark_style] {
                                        (render_cells(&row.children, ctx, false))
                                    }
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

fn render_cells(cells: &[TableCellItem], ctx: &mut RenderContext, is_head: bool) -> Markup {
    html! {
        @for cell_item in cells {
            @match cell_item {
                TableCellItem::Cell(cell) => {
                    @let colspan = utils::extract_text(&cell.x).parse::<usize>().ok().filter(|&n| n > 1);
                    @let rowspan = utils::extract_text(&cell.y).parse::<usize>().ok().filter(|&n| n > 1);
                    @let style = utils::build_style(&cell.parameters);
                    @let class = utils::param_class(&cell.parameters);
                    @let dark_style = utils::build_dark_style(&cell.parameters);
                    @if is_head {
                        th class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dark-style=[dark_style] {
                            (render_elements(&cell.children, ctx))
                        }
                    } @else {
                        td class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dark-style=[dark_style] {
                            (render_elements(&cell.children, ctx))
                        }
                    }
                }
                TableCellItem::Conditional(cond) => {
                    @for cell in &cond.cells {
                        @let colspan = utils::extract_text(&cell.x).parse::<usize>().ok().filter(|&n| n > 1);
                        @let rowspan = utils::extract_text(&cell.y).parse::<usize>().ok().filter(|&n| n > 1);
                        @let style = utils::build_style(&cell.parameters);
                        @let class = utils::param_class(&cell.parameters);
                        @let dark_style = utils::build_dark_style(&cell.parameters);
                        @if is_head {
                            th class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dark-style=[dark_style] {
                                (render_elements(&cell.children, ctx))
                            }
                        } @else {
                            td class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dark-style=[dark_style] {
                                (render_elements(&cell.children, ctx))
                            }
                        }
                    }
                }
            }
        }
    }
}
