use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, ListKind, Span};
use winnow::Result;
use winnow::combinator::peek;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::take_while;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListMarker {
    Bullet(char),
    Ordered { kind: ListKind, delimiter: char },
}

struct ListLine {
    indent: usize,
    content_indent: usize,
    marker: ListMarker,
    content: String,
    segments: Vec<SourceSegment>,
    original_content_start: usize,
    original_line_start: usize,
    original_line_end: usize,
}

struct ListNode {
    line_index: usize,
    content_indent: usize,
    children: Vec<usize>,
}

/// Parses a contiguous markdown list block (`-` / `*` items) from the current line.
pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let lines = collect_list_lines(parser_input)?;

    let (nodes, roots) = build_list_tree(&lines);
    // `collect_list_lines` guarantees all root-level lines share the same marker
    // type, so the roots form exactly one list element. No grouping needed here.
    build_list_element(&lines, &nodes, &roots, parser_input)
}

fn list_marker(parser_input: &mut ParserInput) -> Result<ListMarker> {
    let input: &str = &parser_input.input;
    let Some((marker, marker_len)) = scan_list_marker(input) else {
        return Err(winnow::error::ContextError::new());
    };

    let _: &str = parser_input.next_slice(marker_len);
    Ok(marker)
}

fn scan_list_marker(input: &str) -> Option<(ListMarker, usize)> {
    let bytes = input.as_bytes();
    if bytes.len() < 2 {
        return None;
    }

    if matches!(bytes[0], b'-' | b'+' | b'*') && bytes[1] == b' ' {
        return Some((ListMarker::Bullet(bytes[0] as char), 2));
    }

    let mut digit_end = 0;
    while digit_end < bytes.len() && bytes[digit_end].is_ascii_digit() {
        digit_end += 1;
    }
    if digit_end > 0
        && digit_end + 1 < bytes.len()
        && matches!(bytes[digit_end], b'.' | b')')
        && bytes[digit_end + 1] == b' '
    {
        return Some((
            ListMarker::Ordered {
                kind: ListKind::OrderedNumeric,
                delimiter: bytes[digit_end] as char,
            },
            digit_end + 2,
        ));
    }

    if bytes.len() >= 3
        && bytes[0].is_ascii_lowercase()
        && matches!(bytes[1], b'.' | b')')
        && bytes[2] == b' '
    {
        let kind = if bytes[0] == b'i' {
            ListKind::OrderedRomanLower
        } else {
            ListKind::OrderedAlphaLower
        };
        return Some((
            ListMarker::Ordered {
                kind,
                delimiter: bytes[1] as char,
            },
            3,
        ));
    }

    if bytes.len() >= 3
        && bytes[0].is_ascii_uppercase()
        && matches!(bytes[1], b'.' | b')')
        && bytes[2] == b' '
    {
        let kind = if bytes[0] == b'I' {
            ListKind::OrderedRomanUpper
        } else {
            ListKind::OrderedAlphaUpper
        };
        return Some((
            ListMarker::Ordered {
                kind,
                delimiter: bytes[1] as char,
            },
            3,
        ));
    }

    None
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
    let mut first_line = list_line(parser_input)?;
    consume_lazy_continuation_lines(parser_input, &mut first_line);
    let root_marker = first_line.marker;
    let mut lines = vec![first_line];
    let mut stack = vec![(lines[0].content_indent, lines[0].marker)];

    loop {
        let checkpoint = parser_input.checkpoint();
        let mut line = match list_line(parser_input) {
            Ok(line) => line,
            Err(_) => {
                parser_input.reset(&checkpoint);
                break;
            }
        };

        while let Some((top_content_indent, _)) = stack.last() {
            if line.indent < *top_content_indent {
                stack.pop();
            } else {
                break;
            }
        }

        let is_new_root = stack.is_empty();
        if is_new_root && !is_same_marker_type(root_marker, line.marker) {
            parser_input.reset(&checkpoint);
            break;
        }

        consume_lazy_continuation_lines(parser_input, &mut line);
        stack.push((line.content_indent, line.marker));
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
    let mut segments = Vec::new();
    if !content.is_empty() {
        segments.push(SourceSegment {
            logical_start: 0,
            original_start: content_start,
            len: content.len(),
        });
    }
    line_break_or_eof(parser_input)?;
    let line_end = parser_input.previous_token_end();

    Ok(ListLine {
        indent,
        content_indent: content_start.saturating_sub(line_start),
        marker,
        content: content.to_string(),
        segments,
        original_content_start: content_start,
        original_line_start: line_start,
        original_line_end: line_end,
    })
}

/// Tries to consume one lazy-continuation line, appending its content to `base`
/// in place. Returns `Err` (without consuming input) when the next line is empty,
/// is less-indented than the list content column, or starts a new list marker.
///
/// Continuation rule: strip up to `base.content_indent` leading spaces (the list
/// content column), then take the rest as additional item text. This applies to
/// both plain text and block-like lines, so list lazy-continuation boundaries are
/// consistently defined by marker+whitespace (`*␠`, `-␠`, `1.␠`, ...). List markers
/// are still handled by the outer list-line collector/tree builder.
/// The previous line's `\n` is mapped into logical content as the separator so
/// re-parse offsets stay aligned with the original source.
fn list_lazy_continuation_line(parser_input: &mut ParserInput, base: &mut ListLine) -> Result<()> {
    let remaining: &str = &parser_input.input;
    if remaining.is_empty() {
        return Err(winnow::error::ContextError::new());
    }
    let line_end_pos = remaining.find('\n').unwrap_or(remaining.len());
    let line = &remaining[..line_end_pos];
    let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
    let after_spaces = line.trim_start_matches(' ');
    if after_spaces.is_empty() {
        return Err(winnow::error::ContextError::new());
    }
    if line_starts_list(after_spaces) {
        return Err(winnow::error::ContextError::new());
    }
    if leading_spaces < base.content_indent {
        return Err(winnow::error::ContextError::new());
    }

    // The previous line consumed its trailing '\n' (or hit EOF). Since
    // `remaining` is non-empty here, there was a '\n' immediately before this
    // line — its original offset is `original_line_end - 1`.
    let separator_original = base.original_line_end.saturating_sub(1);
    let separator_logical = base.content.len();

    let _: &str =
        take_while(0..=base.content_indent, |c: char| c == ' ').parse_next(parser_input)?;
    let cont_content_start = parser_input.current_token_start();
    let cont_content = line_content(parser_input)?;
    line_break_or_eof(parser_input)?;
    let line_end = parser_input.previous_token_end();

    base.content.push('\n');
    base.segments.push(SourceSegment {
        logical_start: separator_logical,
        original_start: separator_original,
        len: 1,
    });
    if !cont_content.is_empty() {
        let logical_start = base.content.len();
        base.segments.push(SourceSegment {
            logical_start,
            original_start: cont_content_start,
            len: cont_content.len(),
        });
        base.content.push_str(cont_content);
    }
    base.original_line_end = line_end;

    Ok(())
}

fn consume_lazy_continuation_lines(parser_input: &mut ParserInput, base: &mut ListLine) {
    loop {
        let checkpoint = parser_input.checkpoint();
        if list_lazy_continuation_line(parser_input, base).is_err() {
            parser_input.reset(&checkpoint);
            break;
        }
    }
}

/// Returns whether a line starts a list marker. These remain handled by the
/// list-line collector/tree builder, not by lazy continuation.
fn line_starts_list(after_spaces: &str) -> bool {
    scan_list_marker(after_spaces).is_some()
}

/// Builds a parent/child tree from list lines using content-column indentation.
///
/// Invariant: the stack contains the current ancestor path with strictly increasing
/// content columns, so the nearest remaining stack top is the parent candidate.
fn build_list_tree(lines: &[ListLine]) -> (Vec<ListNode>, Vec<usize>) {
    let mut nodes: Vec<ListNode> = Vec::with_capacity(lines.len());
    let mut roots = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        while let Some(&top_index) = stack.last() {
            let top_content_indent = nodes[top_index].content_indent;
            if line.indent < top_content_indent {
                stack.pop();
            } else {
                break;
            }
        }
        let parent = stack.last().copied();

        let node_index = nodes.len();
        nodes.push(ListNode {
            line_index,
            content_indent: line.content_indent,
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

/// Re-parses list item text as a nested document while preserving original source
/// offsets. This allows block constructs (e.g. nested `>`, `---`) inside list items.
fn parse_item_content(line: &ListLine, parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    let mut child_input = ParserInput {
        input: InputSource::new_segmented(
            &line.content,
            line.segments.clone(),
            line.original_content_start,
        ),
        state: parser_input.state.clone(),
    };
    let previous_block_mode = child_input
        .state
        .replace_block_mode(BlockMode::NestedDocument);
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
