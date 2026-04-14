//! Table rendering

use maud::{Markup, html};
use sevenmark_ast::{Parameters, Span, TableCellItem, TableRowElement, TableRowItem};

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, sanitize, utils};

pub fn render(
    span: &Span,
    parameters: &Parameters,
    children: &[TableRowItem],
    ctx: &mut RenderContext,
) -> Markup {
    ctx.enter_suppress_soft_breaks();

    let style = utils::build_style(parameters);
    let class = utils::merge_class(classes::TABLE, parameters);
    let wrapper_align_class = match utils::get_param(parameters, "align")
        .map(|value| value.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("left") => Some(classes::TABLE_ALIGN_LEFT),
        Some("center") => Some(classes::TABLE_ALIGN_CENTER),
        Some("right") => Some(classes::TABLE_ALIGN_RIGHT),
        _ => None,
    };

    // `#width` mirrors namuWiki's `<tablewidth=N>`: the value goes on the wrapper div
    // (so float and width live on the same element).  The inner table fills the
    // wrapper via CSS (`.sm-table { width: 100% }`).
    let wrapper_width_style = utils::get_param(parameters, "width")
        .map(|w| sanitize::sanitize_inline_style(&format!("width:{}", w)))
        .filter(|s| !s.is_empty());

    let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(parameters));
    let caption = utils::get_param(parameters, "caption");
    let sortable = parameters.contains_key("sortable");

    // Partition rows into head and body.
    // A row is a head row if it has the `#head` flag parameter, even when it
    // originated inside a conditional branch.
    let mut head_rows: Vec<&TableRowElement> = Vec::new();
    let mut body_rows: Vec<&TableRowElement> = Vec::new();

    for item in children {
        match item {
            TableRowItem::Row(row) => {
                if row.parameters.contains_key("head") {
                    head_rows.push(row);
                } else {
                    body_rows.push(row);
                }
            }
            TableRowItem::Conditional(cond) => {
                for row in &cond.rows {
                    if row.parameters.contains_key("head") {
                        head_rows.push(row);
                    } else {
                        body_rows.push(row);
                    }
                }
            }
        }
    }

    let content = html! {
        (dark_tag)
        div
            class=(match wrapper_align_class {
                Some(align_class) => format!("{} {}", classes::TABLE_WRAPPER, align_class),
                None => classes::TABLE_WRAPPER.to_string(),
            })
            style=[wrapper_width_style]
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        {
            table
                class=(class)
                style=[style]
                data-dk=[dk]
                data-sortable=[sortable.then_some("true")]
            {
                @if let Some(cap) = caption {
                    caption { (cap) }
                }
                @if !head_rows.is_empty() {
                    thead {
                        @for row in &head_rows {
                            (render_row(row, ctx, true))
                        }
                    }
                }
                tbody {
                    @for row in &body_rows {
                        (render_row(row, ctx, false))
                    }
                }
            }
        }
    };

    ctx.exit_suppress_soft_breaks();
    content
}

fn render_row(row: &TableRowElement, ctx: &mut RenderContext, is_head: bool) -> Markup {
    let row_style = utils::build_style(&row.parameters);
    let row_class = utils::param_class(&row.parameters);
    let (row_dk, row_dark_tag) = utils::dark_style_parts(utils::build_dark_style(&row.parameters));

    html! {
        (row_dark_tag)
        tr class=[row_class] style=[row_style] data-dk=[row_dk] {
            (render_cells(&row.children, ctx, is_head))
        }
    }
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
                    @let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(&cell.parameters));
                    (dark_tag)
                    @if is_head {
                        th class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dk=[dk] {
                            (render_elements(&cell.children, ctx))
                        }
                    } @else {
                        td class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dk=[dk] {
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
                        @let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(&cell.parameters));
                        (dark_tag)
                        @if is_head {
                            th class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dk=[dk] {
                                (render_elements(&cell.children, ctx))
                            }
                        } @else {
                            td class=[class] colspan=[colspan] rowspan=[rowspan] style=[style] data-dk=[dk] {
                                (render_elements(&cell.children, ctx))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_support::{parse_fragment, render_html, selector};

    #[test]
    fn conditional_head_rows_render_inside_thead() {
        let input = r#"{{{#table
{{{#if true ::
[[#head [[Name]] [[Value]]]]
}}}
[[[[Alice]] [[1]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        assert!(
            doc.select(&selector("table.sm-table thead"))
                .next()
                .is_some(),
            "expected a table head, got:\n{html}"
        );

        let head_text: Vec<_> = doc
            .select(&selector("table.sm-table thead th"))
            .map(|cell| cell.text().collect::<String>())
            .collect();
        assert_eq!(
            head_text,
            vec!["Name".to_string(), "Value".to_string()],
            "expected conditional #head row to render as header cells, got:\n{html}"
        );
        assert!(
            doc.select(&selector("table.sm-table tbody td"))
                .all(|cell| cell.text().collect::<String>() != "Name"),
            "conditional #head row should not render inside tbody cells, got:\n{html}"
        );
    }

    #[test]
    fn table_align_parameter_applies_wrapper_class() {
        let input = r#"{{{#table #align="right"
[[[[Aligned]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        assert_eq!(
            doc.select(&selector("div.sm-table-wrapper.sm-table-align-right"))
                .count(),
            1,
            "expected right-aligned class on the table wrapper, got:\n{html}"
        );
        assert_eq!(
            doc.select(&selector("table.sm-table.sm-table-align-right"))
                .count(),
            0,
            "alignment class should stay on the wrapper, not the table, got:\n{html}"
        );
    }

    #[test]
    fn width_parameter_applies_to_wrapper_not_table() {
        let input = r#"{{{#table #width="400px"
[[[[Cell]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        let wrapper = doc
            .select(&selector("div.sm-table-wrapper"))
            .next()
            .expect("expected wrapper div");
        let wrapper_style = wrapper.value().attr("style").unwrap_or("");
        assert!(
            wrapper_style.contains("width"),
            "expected width on the wrapper style, got:\n{html}"
        );

        let table = doc
            .select(&selector("table.sm-table"))
            .next()
            .expect("expected table");
        let table_style = table.value().attr("style").unwrap_or("");
        assert!(
            !table_style.contains("400px"),
            "width value should not appear on the inner table style, got:\n{html}"
        );
    }

    #[test]
    fn width_parameter_strips_var_function() {
        // var() is blocked by the sanitizer's security scan.
        let input = r#"{{{#table #width="var(--custom)"
[[[[Cell]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        let wrapper = doc
            .select(&selector("div.sm-table-wrapper"))
            .next()
            .expect("expected wrapper div");
        let wrapper_style = wrapper.value().attr("style").unwrap_or("");
        assert!(
            !wrapper_style.contains("var("),
            "var() in #width should be stripped by the sanitizer, got:\n{html}"
        );
    }

    #[test]
    fn width_parameter_strips_expression_function() {
        // expression() is an IE-era CSS injection vector and is blocked by the sanitizer.
        let input = r#"{{{#table #width="expression(alert(1))"
[[[[Cell]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        let wrapper = doc
            .select(&selector("div.sm-table-wrapper"))
            .next()
            .expect("expected wrapper div");
        let wrapper_style = wrapper.value().attr("style").unwrap_or("");
        assert!(
            !wrapper_style.contains("expression("),
            "expression() in #width should be stripped by the sanitizer, got:\n{html}"
        );
    }

    #[test]
    fn width_and_align_together_on_wrapper() {
        let input = r#"{{{#table #align="right" #width="400px"
[[[[Cell]]]]
}}}"#;

        let html = render_html(input);
        let doc = parse_fragment(&html);

        let wrapper = doc
            .select(&selector("div.sm-table-wrapper.sm-table-align-right"))
            .next()
            .expect("expected right-aligned wrapper div");
        let wrapper_style = wrapper.value().attr("style").unwrap_or("");
        assert!(
            wrapper_style.contains("width"),
            "expected width on the wrapper style when #align and #width are combined, got:\n{html}"
        );
    }
}
