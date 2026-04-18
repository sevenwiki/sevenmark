use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, Span};
use winnow::Result;
use winnow::combinator::{peek, repeat};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

struct ListLine {
    indent: usize,
    content_indent: usize,
    content: String,
    original_content_start: usize,
    original_line_start: usize,
    original_line_end: usize,
}

pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let mut lines = vec![root_list_line(parser_input)?];
    let mut rest: Vec<ListLine> = repeat(0.., list_line).parse_next(parser_input)?;
    lines.append(&mut rest);

    let (list, _) = build_list_element(&lines, 0, parser_input)?;
    Ok(list)
}

fn root_list_line(parser_input: &mut ParserInput) -> Result<ListLine> {
    peek((take_while(0..=3, |c: char| c == ' '), literal("- "))).parse_next(parser_input)?;
    list_line(parser_input)
}

fn list_line(parser_input: &mut ParserInput) -> Result<ListLine> {
    peek((take_while(0.., |c: char| c == ' '), literal("- "))).parse_next(parser_input)?;

    let line_start = parser_input.current_token_start();
    let spaces: &str = take_while(0.., |c: char| c == ' ').parse_next(parser_input)?;
    let indent = spaces.len();

    literal("- ").parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();

    let content = line_content(parser_input)?;
    line_break_or_eof(parser_input)?;
    let line_end = parser_input.previous_token_end();

    Ok(ListLine {
        indent,
        content_indent: indent + 2,
        content: content.to_string(),
        original_content_start: content_start,
        original_line_start: line_start,
        original_line_end: line_end,
    })
}

fn build_list_items(
    lines: &[ListLine],
    parent_content_indent: usize,
    parser_input: &mut ParserInput,
) -> Result<(Vec<ListContentItem>, usize)> {
    let mut items = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = &lines[i];

        if line.indent < parent_content_indent {
            break;
        }

        let item_start = line.original_line_start;
        let mut item_end = line.original_line_end;
        let mut children = parse_item_content(line, parser_input)?;
        i += 1;

        while i < lines.len() && lines[i].indent >= line.content_indent {
            let (nested_list, consumed) =
                build_list_element(&lines[i..], line.content_indent, parser_input)?;
            item_end = nested_list.span().end;
            children.push(nested_list);
            i += consumed;
        }

        items.push(ListContentItem::Item(ListItemElement {
            span: Span {
                start: item_start,
                end: item_end,
            },
            open_span: Span::synthesized(),
            close_span: Span::synthesized(),
            parameters: Default::default(),
            children,
        }));
    }

    Ok((items, i))
}

fn build_list_element(
    lines: &[ListLine],
    parent_content_indent: usize,
    parser_input: &mut ParserInput,
) -> Result<(Element, usize)> {
    let (children, consumed) = build_list_items(lines, parent_content_indent, parser_input)?;
    let start = lines
        .first()
        .map(|line| line.original_line_start)
        .unwrap_or_default();
    let end = lines
        .get(consumed.saturating_sub(1))
        .map(|line| line.original_line_end)
        .unwrap_or(start);

    Ok((
        Element::List(ListElement {
            span: Span { start, end },
            open_span: Span::synthesized(),
            close_span: Span::synthesized(),
            kind: String::new(),
            parameters: Default::default(),
            children,
        }),
        consumed,
    ))
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
