use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, ListKind, Span};
use winnow::Result;
use winnow::combinator::{alt, peek};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListMarker {
    Bullet(char),
    Ordered { kind: ListKind, delimiter: char },
}

struct ListLine {
    indent: usize,
    marker: ListMarker,
    content: String,
    original_content_start: usize,
    original_line_start: usize,
    original_line_end: usize,
}

struct ListNode {
    line_index: usize,
    indent: usize,
    children: Vec<usize>,
}

/// Parses a contiguous markdown list block (`-` / `*` items) from the current line.
pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let lines = collect_list_lines(parser_input)?;

    let (nodes, roots) = build_list_tree(&lines);
    let mut root_lists = build_list_elements(&lines, &nodes, &roots, parser_input)?;
    Ok(root_lists.remove(0))
}

fn list_marker(parser_input: &mut ParserInput) -> Result<ListMarker> {
    alt((
        literal("- ").value(ListMarker::Bullet('-')),
        literal("+ ").value(ListMarker::Bullet('+')),
        literal("* ").value(ListMarker::Bullet('*')),
        (
            take_while(1.., |c: char| c.is_ascii_digit()),
            alt((literal(". "), literal(") "))),
        )
            .map(|(_, delimiter): (&str, &str)| {
                let delimiter = delimiter
                    .chars()
                    .next()
                    .expect("ordered list delimiter must be present");
                ListMarker::Ordered {
                    kind: ListKind::OrderedNumeric,
                    delimiter,
                }
            }),
        (
            take_while(1..=1, |c: char| c.is_ascii_lowercase()),
            alt((literal(". "), literal(") "))),
        )
            .map(|(token, delimiter): (&str, &str)| {
                let delimiter = delimiter
                    .chars()
                    .next()
                    .expect("ordered list delimiter must be present");
                let kind = if token == "i" {
                    ListKind::OrderedRomanLower
                } else {
                    ListKind::OrderedAlphaLower
                };
                ListMarker::Ordered { kind, delimiter }
            }),
        (
            take_while(1..=1, |c: char| c.is_ascii_uppercase()),
            alt((literal(". "), literal(") "))),
        )
            .map(|(token, delimiter): (&str, &str)| {
                let delimiter = delimiter
                    .chars()
                    .next()
                    .expect("ordered list delimiter must be present");
                let kind = if token == "I" {
                    ListKind::OrderedRomanUpper
                } else {
                    ListKind::OrderedAlphaUpper
                };
                ListMarker::Ordered { kind, delimiter }
            }),
    ))
    .parse_next(parser_input)
}

fn is_same_marker_type(left: ListMarker, right: ListMarker) -> bool {
    match (left, right) {
        (ListMarker::Bullet(a), ListMarker::Bullet(b)) => a == b,
        (
            ListMarker::Ordered {
                kind: left_kind,
                delimiter: left_delimiter,
            },
            ListMarker::Ordered {
                kind: right_kind,
                delimiter: right_delimiter,
            },
        ) => left_kind == right_kind && left_delimiter == right_delimiter,
        _ => false,
    }
}

fn collect_list_lines(parser_input: &mut ParserInput) -> Result<Vec<ListLine>> {
    let first_line = list_line(parser_input)?;
    let root_marker = first_line.marker;
    let mut lines = vec![first_line];
    let mut stack = vec![(lines[0].indent, lines[0].marker)];

    loop {
        let checkpoint = parser_input.checkpoint();
        let state = parser_input.state.clone();
        let line = match list_line(parser_input) {
            Ok(line) => line,
            Err(_) => {
                parser_input.reset(&checkpoint);
                parser_input.state = state;
                break;
            }
        };

        while let Some((top_indent, _)) = stack.last() {
            if *top_indent >= line.indent {
                stack.pop();
            } else {
                break;
            }
        }

        let is_new_root = stack.is_empty();
        if is_new_root && !is_same_marker_type(root_marker, line.marker) {
            parser_input.reset(&checkpoint);
            parser_input.state = state;
            break;
        }

        stack.push((line.indent, line.marker));
        lines.push(line);
    }

    Ok(lines)
}

fn list_line(parser_input: &mut ParserInput) -> Result<ListLine> {
    peek((take_while(0.., |c: char| c == ' '), list_marker)).parse_next(parser_input)?;

    let line_start = parser_input.current_token_start();
    let spaces: &str = take_while(0.., |c: char| c == ' ').parse_next(parser_input)?;
    let indent = spaces.len();

    let marker = list_marker(parser_input)?;
    let content_start = parser_input.current_token_start();

    let content = line_content(parser_input)?;
    line_break_or_eof(parser_input)?;
    let line_end = parser_input.previous_token_end();

    Ok(ListLine {
        indent,
        marker,
        content: content.to_string(),
        original_content_start: content_start,
        original_line_start: line_start,
        original_line_end: line_end,
    })
}

/// Builds a parent/child tree from list lines using indentation.
///
/// Invariant: the stack contains the current ancestor path with strictly increasing
/// indentation, so the nearest remaining stack top is the parent candidate.
fn build_list_tree(lines: &[ListLine]) -> (Vec<ListNode>, Vec<usize>) {
    let mut nodes: Vec<ListNode> = Vec::with_capacity(lines.len());
    let mut roots = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            let top_indent = nodes[top_index].indent;
            if top_indent >= line.indent {
                stack.pop();
            } else {
                break;
            }
        }
        let parent = stack.last().copied();

        let node_index = nodes.len();
        nodes.push(ListNode {
            line_index,
            indent: line.indent,
            children: Vec::new(),
        });

        if let Some(parent_index) = parent {
            nodes[parent_index].children.push(node_index);
        } else {
            roots.push(node_index);
        }

        stack.push(node_index);
    }

    (nodes, roots)
}

fn group_by_marker(
    lines: &[ListLine],
    nodes: &[ListNode],
    node_indices: &[usize],
) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();

    for &node_index in node_indices {
        if let Some(current_group) = groups.last_mut() {
            let last_index = *current_group
                .last()
                .expect("group must contain at least one index");
            let left = lines[nodes[last_index].line_index].marker;
            let right = lines[nodes[node_index].line_index].marker;
            if is_same_marker_type(left, right) {
                current_group.push(node_index);
                continue;
            }
        }
        groups.push(vec![node_index]);
    }

    groups
}

/// Builds one or more `List` elements from tree node indices, splitting adjacent
/// siblings when their marker types differ (CommonMark list-type boundary rule).
fn build_list_elements(
    lines: &[ListLine],
    nodes: &[ListNode],
    node_indices: &[usize],
    parser_input: &mut ParserInput,
) -> Result<Vec<Element>> {
    let groups = group_by_marker(lines, nodes, node_indices);
    let mut result = Vec::with_capacity(groups.len());
    for group in groups {
        result.push(build_list_element(lines, nodes, &group, parser_input)?);
    }
    Ok(result)
}

fn list_kind_for_marker(marker: ListMarker) -> ListKind {
    match marker {
        ListMarker::Ordered { kind, .. } => kind,
        ListMarker::Bullet(_) => ListKind::Unordered,
    }
}

/// Builds a single `List` element from a homogeneous marker group.
fn build_list_element(
    lines: &[ListLine],
    nodes: &[ListNode],
    node_indices: &[usize],
    parser_input: &mut ParserInput,
) -> Result<Element> {
    let mut children = Vec::with_capacity(node_indices.len());
    let start = node_indices
        .first()
        .map(|&node_index| lines[nodes[node_index].line_index].original_line_start)
        .unwrap_or_default();
    let mut end = start;
    let kind = node_indices
        .first()
        .map(|&node_index| list_kind_for_marker(lines[nodes[node_index].line_index].marker))
        .unwrap_or(ListKind::Unordered);

    for &node_index in node_indices {
        let item = build_list_item(lines, nodes, node_index, parser_input)?;
        end = item.span.end;
        children.push(ListContentItem::Item(item));
    }

    Ok(Element::List(ListElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        kind,
        parameters: Default::default(),
        children,
    }))
}

/// Builds a single `ListItem` and recursively appends nested child lists.
fn build_list_item(
    lines: &[ListLine],
    nodes: &[ListNode],
    node_index: usize,
    parser_input: &mut ParserInput,
) -> Result<ListItemElement> {
    let node = &nodes[node_index];
    let line = &lines[node.line_index];
    let item_start = line.original_line_start;
    let mut item_end = line.original_line_end;
    let mut children = parse_item_content(line, parser_input)?;

    if !node.children.is_empty() {
        let nested_lists = build_list_elements(lines, nodes, &node.children, parser_input)?;
        for nested_list in nested_lists {
            item_end = nested_list.span().end;
            children.push(nested_list);
        }
    }

    Ok(ListItemElement {
        span: Span {
            start: item_start,
            end: item_end,
        },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        parameters: Default::default(),
        children,
    })
}

/// Re-parses list item text in inline mode while preserving original source offsets.
fn parse_item_content(line: &ListLine, parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    let mut child_input = ParserInput {
        input: InputSource::new_segmented(
            &line.content,
            if line.content.is_empty() {
                Vec::new()
            } else {
                vec![SourceSegment {
                    logical_start: 0,
                    original_start: line.original_content_start,
                    len: line.content.len(),
                }]
            },
            line.original_content_start,
        ),
        state: parser_input.state.clone(),
    };
    let previous_block_mode = child_input
        .state
        .replace_block_mode(BlockMode::InlineContent);
    child_input
        .state
        .increase_depth()
        .map_err(|e| e.into_context_error())?;
    let children = parse_document_input(&mut child_input);
    child_input.state.decrease_depth();
    child_input.state.replace_block_mode(previous_block_mode);
    parser_input.state = child_input.state;
    Ok(children)
}
