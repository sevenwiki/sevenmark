use crate::core::parse_document;
use crate::parser::ParserInput;
use crate::parser::utils::{SegmentTable, remap_offset};
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, Span, remap::remap_elements};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, eof, peek, repeat, terminated};
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

pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    if parser_input.state.current_depth() > 0 {
        return Err(winnow::error::ContextError::new());
    }

    let current_pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(current_pos) {
        return Err(winnow::error::ContextError::new());
    }

    let start = current_pos;

    let lines: Vec<ListLine> = repeat(1.., list_line).parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    let base_indent = lines[0].indent;
    let items = build_items(&lines, base_indent);

    Ok(Element::List(ListElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        kind: String::new(),
        parameters: Default::default(),
        children: items.into_iter().map(ListContentItem::Item).collect(),
    }))
}

fn list_line(parser_input: &mut ParserInput) -> Result<ListLine> {
    let pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(pos) {
        return Err(winnow::error::ContextError::new());
    }

    // Peek before consuming — ensures safe backtracking if not a list line.
    peek((
        take_while(0.., |c: char| c == ' '),
        literal("- "),
    ))
    .parse_next(parser_input)?;

    let line_start = pos;

    let spaces: &str = take_while(0.., |c: char| c == ' ').parse_next(parser_input)?;
    let indent = spaces.len();

    literal("- ").parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();

    let content: &str = terminated(
        take_while(0.., |c: char| c != '\n'),
        alt((line_ending, eof)),
    )
    .parse_next(parser_input)?;

    let line_end = parser_input.previous_token_end();

    Ok(ListLine {
        indent,
        content: content.to_string(),
        original_content_start: content_start,
        original_line_start: line_start,
        original_line_end: line_end,
    })
}

fn build_items(lines: &[ListLine], base_indent: usize) -> Vec<ListItemElement> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].indent < base_indent {
            break;
        }
        if lines[i].indent != base_indent {
            i += 1;
            continue;
        }

        // Collect body lines (deeper indent) that belong to this item.
        let body_start = i + 1;
        let mut body_end = body_start;
        while body_end < lines.len() && lines[body_end].indent > base_indent {
            body_end += 1;
        }
        let body = &lines[body_start..body_end];

        let item_original_start = lines[i].original_line_start;
        let item_original_end = body.last().map(|l| l.original_line_end).unwrap_or(lines[i].original_line_end);

        let (content, segments) = build_item_content(&lines[i], body, base_indent);
        let mut children = parse_document(&content);
        remap_elements(&mut children, &|off| remap_offset(off, &segments));

        result.push(ListItemElement {
            span: Span { start: item_original_start, end: item_original_end },
            open_span: Span::synthesized(),
            close_span: Span::synthesized(),
            parameters: Default::default(),
            children,
        });

        i = body_end;
    }

    result
}

/// Reconstructs an item's full content string and the corresponding segment table.
/// The content string is: `direct_content\nbody_line_0\nbody_line_1\n...`
/// where body lines have `(base_indent + 2)` leading spaces stripped.
fn build_item_content(
    item: &ListLine,
    body: &[ListLine],
    base_indent: usize,
) -> (String, SegmentTable) {
    let mut content = String::new();
    let mut segments: SegmentTable = Vec::with_capacity(1 + body.len());

    segments.push((0, item.original_content_start));
    content.push_str(&item.content);

    let strip = base_indent + 2;
    for line in body {
        content.push('\n');
        let seg_start = content.len();
        let stripped_indent = line.indent.saturating_sub(strip);
        // After stripping `strip` spaces, the remaining content starts at:
        let orig_start = line.original_line_start + strip.min(line.indent);
        segments.push((seg_start, orig_start));
        // Reconstruct: remaining_spaces + "- " + content
        for _ in 0..stripped_indent {
            content.push(' ');
        }
        content.push_str("- ");
        content.push_str(&line.content);
    }

    (content, segments)
}