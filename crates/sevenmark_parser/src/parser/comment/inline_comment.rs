use crate::parser::ParserInput;
use crate::parser::utils::{line_content, line_end};
use sevenmark_ast::{CommentElement, Element, Span};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse inline comments starting with "//"
/// Comments continue until end of line or end of file
/// Only allowed at top-level (recursion_depth == 0) to prevent `//` in URLs
/// and other content from being greedily consumed as comments inside
/// nested constructs like [[ ]], {{{ }}}, etc.
pub fn inline_comment_parser(parser_input: &mut ParserInput) -> Result<Element> {
    if parser_input.state.recursion_depth > 0 {
        return Err(winnow::error::ContextError::new());
    }

    let start = parser_input.current_token_start();

    literal("//").parse_next(parser_input)?;
    let content = line_content(parser_input)?.to_string();
    line_end(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(Element::Comment(CommentElement {
        span: Span { start, end },
        value: content,
    }))
}
