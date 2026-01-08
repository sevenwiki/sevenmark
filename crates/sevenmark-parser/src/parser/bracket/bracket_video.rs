use crate::ast::{AstNode, Location, NodeKind};
use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use winnow::Result;
use winnow::combinator::{alt, delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse video elements: [[#youtube ...]], [[#vimeo ...]], [[#nicovideo ...]]
pub fn bracket_video_parser(parser_input: &mut ParserInput) -> Result<AstNode> {
    let start = parser_input.input.current_token_start();

    let (provider, parameters) = delimited(
        literal("[["),
        (video_provider_parser, opt(parameter_core_parser)),
        literal("]]"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(AstNode::new(
        Location { start, end },
        NodeKind::Video {
            provider: provider.to_string(),
            parameters: parameters.unwrap_or_default(),
        },
    ))
}

/// Parse video provider tag: #youtube, #vimeo, #nicovideo
fn video_provider_parser<'a>(input: &mut ParserInput<'a>) -> Result<&'a str> {
    alt((
        literal("#youtube").map(|_| "youtube"),
        literal("#vimeo").map(|_| "vimeo"),
        literal("#nicovideo").map(|_| "nicovideo"),
    ))
    .parse_next(input)
}
