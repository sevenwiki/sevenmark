use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, Span};
use winnow::Result;
use winnow::combinator::{alt, peek, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

struct ListLine {
    indent: usize,
    content: String,
    original_content_start: usize,
    original_line_start: usize,
    original_line_end: usize,
}

struct ListNode {
    line_index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let mut lines = vec![list_line(parser_input)?];
    let mut rest: Vec<ListLine> = repeat(0.., list_line).parse_next(parser_input)?;
    lines.append(&mut rest);

    let (nodes, roots) = build_list_tree(&lines);
    build_list_element(&lines, &nodes, &roots, parser_input)
}

fn list_line(parser_input: &mut ParserInput) -> Result<ListLine> {
    peek((
        take_while(0.., |c: char| c == ' '),
        alt((literal("- "), literal("* "))),
    ))
    .parse_next(parser_input)?;

    let line_start = parser_input.current_token_start();
    let spaces: &str = take_while(0.., |c: char| c == ' ').parse_next(parser_input)?;
    let indent = spaces.len();

    alt((literal("- "), literal("* "))).parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();

    let content = line_content(parser_input)?;
    line_break_or_eof(parser_input)?;
    let line_end = parser_input.previous_token_end();

    Ok(ListLine {
        indent,
        content: content.to_string(),
        original_content_start: content_start,
        original_line_start: line_start,
        original_line_end: line_end,
    })
}

fn build_list_tree(lines: &[ListLine]) -> (Vec<ListNode>, Vec<usize>) {
    let mut nodes: Vec<ListNode> = Vec::with_capacity(lines.len());
    let mut roots = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        let parent = if let Some(&previous_index) = stack.last() {
            let previous_indent = lines[nodes[previous_index].line_index].indent;
            if line.indent > previous_indent {
                Some(previous_index)
            } else {
                while let Some(&top_index) = stack.last() {
                    let top_indent = lines[nodes[top_index].line_index].indent;
                    if top_indent > line.indent {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.pop().and_then(|top_index| nodes[top_index].parent)
            }
        } else {
            None
        };

        let node_index = nodes.len();
        nodes.push(ListNode {
            line_index,
            parent,
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

    for &node_index in node_indices {
        let item = build_list_item(lines, nodes, node_index, parser_input)?;
        end = item.span.end;
        children.push(ListContentItem::Item(item));
    }

    Ok(Element::List(ListElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        kind: String::new(),
        parameters: Default::default(),
        children,
    }))
}

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
        let nested_list = build_list_element(lines, nodes, &node.children, parser_input)?;
        item_end = nested_list.span().end;
        children.push(nested_list);
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
