use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{BlockQuoteElement, Element, Span};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

/// Parses contiguous `>` lines as one blockquote, then re-parses the inner content
/// as a nested document while preserving original offset mappings.
pub fn markdown_blockquote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    let raw_lines = collect_blockquote_lines(parser_input)?;

    let end = parser_input.previous_token_end();

    let empty_original_offset = raw_lines
        .first()
        .map(|line| line.original_start)
        .unwrap_or(start);
    let mut logical = String::new();
    let mut segments = Vec::with_capacity(raw_lines.len());
    for line in &raw_lines {
        let logical_start = logical.len();
        logical.push_str(&line.content);
        segments.extend(line.segments.iter().map(|segment| SourceSegment {
            logical_start: logical_start + segment.logical_start,
            original_start: segment.original_start,
            len: segment.len,
        }));
    }

    let mut child_input = ParserInput {
        input: InputSource::new_segmented(&logical, segments, empty_original_offset),
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

    Ok(Element::BlockQuote(BlockQuoteElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        parameters: Default::default(),
        children,
    }))
}

struct BlockQuoteLine {
    content: String,
    content_indent: usize,
    original_start: usize,
    segments: Vec<SourceSegment>,
}

fn collect_blockquote_lines(parser_input: &mut ParserInput) -> Result<Vec<BlockQuoteLine>> {
    let first_line = blockquote_line(parser_input)?;
    let mut content_indent = first_line.content_indent;
    let mut lines = vec![first_line];

    loop {
        let checkpoint = parser_input.checkpoint();
        match blockquote_line(parser_input) {
            Ok(line) => {
                content_indent = line.content_indent;
                lines.push(line);
                continue;
            }
            Err(_) => {
                parser_input.reset(&checkpoint);
            }
        }

        let checkpoint = parser_input.checkpoint();
        match blockquote_lazy_continuation_line(parser_input, content_indent) {
            Ok(line) => lines.push(line),
            Err(_) => {
                parser_input.reset(&checkpoint);
                break;
            }
        }
    }

    Ok(lines)
}

/// Parses one blockquote line and records source segments for both text and line ending.
fn blockquote_line(parser_input: &mut ParserInput) -> Result<BlockQuoteLine> {
    let line_start = parser_input.current_token_start();
    literal(">").parse_next(parser_input)?;
    opt(literal(" ")).parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();

    let content = line_content(parser_input)?;
    let mut logical_content = content.to_string();
    let mut segments = Vec::new();

    if !content.is_empty() {
        segments.push(SourceSegment {
            logical_start: 0,
            original_start: content_start,
            len: content.len(),
        });
    }

    let ending_start = parser_input.current_token_start();
    if let Some(ending) = line_break_or_eof(parser_input)? {
        let logical_start = logical_content.len();
        logical_content.push_str(ending);
        segments.push(SourceSegment {
            logical_start,
            original_start: ending_start,
            len: ending.len(),
        });
    }

    Ok(BlockQuoteLine {
        content: logical_content,
        content_indent: content_start.saturating_sub(line_start),
        original_start: content_start,
        segments,
    })
}

fn blockquote_lazy_continuation_line(
    parser_input: &mut ParserInput,
    content_indent: usize,
) -> Result<BlockQuoteLine> {
    let remaining: &str = &parser_input.input;
    if remaining.is_empty() {
        return Err(winnow::error::ContextError::new());
    }

    let line_end_pos = remaining.find('\n').unwrap_or(remaining.len());
    let line = &remaining[..line_end_pos];
    let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
    if leading_spaces < content_indent || line.trim_start_matches(' ').is_empty() {
        return Err(winnow::error::ContextError::new());
    }

    let _: &str = take_while(0..=content_indent, |c: char| c == ' ').parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();
    let content = line_content(parser_input)?;
    let mut logical_content = content.to_string();
    let mut segments = Vec::new();

    if !content.is_empty() {
        segments.push(SourceSegment {
            logical_start: 0,
            original_start: content_start,
            len: content.len(),
        });
    }

    let ending_start = parser_input.current_token_start();
    if let Some(ending) = line_break_or_eof(parser_input)? {
        let logical_start = logical_content.len();
        logical_content.push_str(ending);
        segments.push(SourceSegment {
            logical_start,
            original_start: ending_start,
            len: ending.len(),
        });
    }

    Ok(BlockQuoteLine {
        content: logical_content,
        content_indent,
        original_start: content_start,
        segments,
    })
}
