use crate::context::BlockMode;
use crate::parser::ParserInput;
use crate::parser::element::content_element_parser;
use crate::parser::markdown::{
    markdown_blockquote_parser, markdown_header_parser, markdown_hline_parser, markdown_list_parser,
};
use sevenmark_ast::Element;
use winnow::Result;
use winnow::combinator::{alt, opt};
use winnow::prelude::*;
use winnow::stream::Stream;

/// Parses a document as a sequence of line-level blocks and inline content.
pub fn block_document_parser(parser_input: &mut ParserInput) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while !parser_input.input.is_empty() {
        if parser_input.input.is_at_line_start() {
            if let Some(element) = parse_line_block(parser_input)? {
                elements.push(element);
                continue;
            }
        }

        let checkpoint = parser_input.checkpoint();
        let state = parser_input.state.clone();
        match content_element_parser(parser_input) {
            Ok(element) => elements.push(element),
            Err(_) => {
                parser_input.reset(&checkpoint);
                parser_input.state = state;
                break;
            }
        }
    }

    Ok(elements)
}

/// Parses block constructs allowed at line start for the current block mode.
fn parse_line_block(parser_input: &mut ParserInput) -> Result<Option<Element>> {
    match parser_input.state.block_mode {
        BlockMode::FullDocument => opt(alt((
            markdown_header_parser,
            markdown_blockquote_parser,
            markdown_hline_parser,
            markdown_list_parser,
        )))
        .parse_next(parser_input),
        BlockMode::NestedDocument => opt(alt((
            markdown_blockquote_parser,
            markdown_hline_parser,
            markdown_list_parser,
        )))
        .parse_next(parser_input),
        BlockMode::InlineContent => Ok(None),
    }
}
