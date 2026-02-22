use crate::parser::ParserInput;
use sevenmark_ast::{CommentElement, Element, Span};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, eof, opt, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_till};

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

    let (_, content_opt) = (
        literal("//"),
        terminated(
            opt(take_till(0.., |c: char| c == '\n')),
            alt((line_ending, eof)),
        ),
    )
        .parse_next(parser_input)?;

    let end = parser_input.previous_token_end();
    let content = content_opt.unwrap_or("").to_string();

    Ok(Element::Comment(CommentElement {
        span: Span { start, end },
        value: content,
    }))
}
