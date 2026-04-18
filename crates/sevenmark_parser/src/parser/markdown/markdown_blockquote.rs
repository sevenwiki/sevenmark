use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_break_or_eof, line_content};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{BlockQuoteElement, Element, Span};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::literal;

/// Parses contiguous `>` lines as one blockquote, then re-parses the inner content
/// as a nested document while preserving original offset mappings.
pub fn markdown_blockquote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    let raw_lines = collect_blockquote_lines(parser_input)?;

    let end = parser_input.previous_token_end();

    let empty_original_offset = raw_lines
        .first()
        .map(|line| line.content_start)
        .unwrap_or(start);
    let logical_len = raw_lines
        .iter()
        .map(|line| line.content.len() + line.ending.map(str::len).unwrap_or_default())
        .sum();
    let mut logical = String::with_capacity(logical_len);
    let mut segments = Vec::with_capacity(raw_lines.len() * 2);
    for line in &raw_lines {
        if !line.content.is_empty() {
            segments.push(SourceSegment {
                logical_start: logical.len(),
                original_start: line.content_start,
                len: line.content.len(),
            });
        }
        logical.push_str(&line.content);

        if let Some(ending) = line.ending {
            if let Some(ending_start) = line.ending_start {
                segments.push(SourceSegment {
                    logical_start: logical.len(),
                    original_start: ending_start,
                    len: ending.len(),
                });
            }
            logical.push_str(ending);
        }
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

struct BlockQuoteLine<'i> {
    content: &'i str,
    ending: Option<&'i str>,
    content_indent: usize,
    content_start: usize,
    ending_start: Option<usize>,
}

fn collect_blockquote_lines<'i>(
    parser_input: &mut ParserInput<'i>,
) -> Result<Vec<BlockQuoteLine<'i>>> {
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
fn blockquote_line<'i>(parser_input: &mut ParserInput<'i>) -> Result<BlockQuoteLine<'i>> {
    let line_start = parser_input.current_token_start();
    literal(">").parse_next(parser_input)?;
    opt(literal(" ")).parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();

    let content = line_content(parser_input)?;

    let ending_start = parser_input.current_token_start();
    let ending = line_break_or_eof(parser_input)?;

    Ok(BlockQuoteLine {
        content,
        ending,
        content_indent: content_start.saturating_sub(line_start),
        content_start,
        ending_start: ending.map(|_| ending_start),
    })
}

fn blockquote_lazy_continuation_line<'i>(
    parser_input: &mut ParserInput<'i>,
    content_indent: usize,
) -> Result<BlockQuoteLine<'i>> {
    let remaining: &str = &parser_input.input;
    if remaining.is_empty() {
        return Err(winnow::error::ContextError::new());
    }

    let has_content_indent = match remaining.as_bytes().get(..content_indent) {
        Some(prefix) => prefix.iter().all(|&b| b == b' '),
        None => false,
    };
    if !has_content_indent {
        return Err(winnow::error::ContextError::new());
    }

    let after_indent = &remaining[content_indent..];
    let extra_spaces = after_indent.bytes().take_while(|&b| b == b' ').count();
    let after_spaces = &after_indent[extra_spaces..];
    if after_spaces.is_empty() || after_spaces.as_bytes().first() == Some(&b'\n') {
        return Err(winnow::error::ContextError::new());
    }

    // Policy: SevenMark keeps blockquote lazy continuation permissive.
    // If indentation matches `content_indent`, any non-empty line is part of the quote
    // even when it looks like a block starter (`- item`, `---`, nested list/table starts).
    // Those lines are intentionally re-parsed inside the quote, not at the root level.

    let _: &str = parser_input.next_slice(content_indent);
    let content_start = parser_input.current_token_start();
    let content = line_content(parser_input)?;

    let ending_start = parser_input.current_token_start();
    let ending = line_break_or_eof(parser_input)?;

    Ok(BlockQuoteLine {
        content,
        ending,
        content_indent,
        content_start,
        ending_start: ending.map(|_| ending_start),
    })
}
