use crate::core::parse_document;
use crate::parser::ParserInput;
use crate::parser::utils::{SegmentTable, remap_offset};
use sevenmark_ast::{BlockQuoteElement, Element, Span, remap::remap_elements};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, eof, opt, repeat, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

pub fn markdown_blockquote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    if parser_input.state.current_depth() > 0 {
        return Err(winnow::error::ContextError::new());
    }

    let current_pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(current_pos) {
        return Err(winnow::error::ContextError::new());
    }

    let start = current_pos;

    let raw_lines: Vec<(String, usize)> =
        repeat(1.., blockquote_line).parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    let mut segments: SegmentTable = Vec::with_capacity(raw_lines.len());
    let mut stripped = String::new();
    for (content, orig_start) in &raw_lines {
        segments.push((stripped.len(), *orig_start));
        stripped.push_str(content);
        stripped.push('\n');
    }

    let mut children = parse_document(&stripped);
    remap_elements(&mut children, &|off| remap_offset(off, &segments));

    Ok(Element::BlockQuote(BlockQuoteElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        parameters: Default::default(),
        children,
    }))
}

/// Returns (stripped_content, original_content_start).
fn blockquote_line(parser_input: &mut ParserInput) -> Result<(String, usize)> {
    let pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(pos) {
        return Err(winnow::error::ContextError::new());
    }

    literal(">").parse_next(parser_input)?;
    opt(literal(" ")).parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();

    let content: &str = terminated(
        take_while(0.., |c: char| c != '\n'),
        alt((line_ending, eof)),
    )
    .parse_next(parser_input)?;

    Ok((content.to_string(), content_start))
}
