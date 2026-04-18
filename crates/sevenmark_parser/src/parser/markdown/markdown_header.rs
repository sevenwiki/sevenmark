use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::utils::{line_content, line_end};
use crate::parser::{InputSource, ParserInput, SourceSegment};
use sevenmark_ast::{Element, HeaderElement, Span};
use winnow::Result;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

/// 헤더 파서 - # Header (1-6개의 # 지원, ! 폴딩 지원)
pub fn markdown_header_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();
    let header_marks: &str = take_while(1..=6, '#').parse_next(parser_input)?;
    let is_folded = opt(literal('!')).parse_next(parser_input)?.is_some();
    opt(literal(' ')).parse_next(parser_input)?;

    let content_start = parser_input.current_token_start();
    let content = line_content(parser_input)?;
    line_end(parser_input)?;

    let end = parser_input.previous_token_end();
    let header_level = header_marks.len();
    let section_index = parser_input.state.next_section_index();
    let children = parse_header_content(content, content_start, parser_input)?;

    Ok(Element::Header(HeaderElement {
        span: Span { start, end },
        level: header_level,
        is_folded,
        section_index,
        children,
    }))
}

fn parse_header_content(
    content: &str,
    content_start: usize,
    parser_input: &mut ParserInput,
) -> Result<Vec<Element>> {
    let mut child_input = ParserInput {
        input: InputSource::new_segmented(
            content,
            if content.is_empty() {
                Vec::new()
            } else {
                vec![SourceSegment {
                    logical_start: 0,
                    original_start: content_start,
                    len: content.len(),
                }]
            },
            content_start,
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
