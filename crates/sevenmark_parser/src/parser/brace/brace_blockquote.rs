use crate::context::BlockMode;
use crate::core::parse_document_input;
use crate::parser::InputSource;
use crate::parser::ParserInput;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::parse_raw_until_balanced_triple_brace;
use sevenmark_ast::{BlockQuoteElement, Element, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_blockquote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    literal("{{{#quote").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;
    let content_start = parser_input.current_token_start();
    let raw_content = parse_raw_until_balanced_triple_brace(parser_input)?;
    let content = raw_content
        .value
        .trim_end_matches(|c: char| c.is_ascii_whitespace());

    let mut child_input = ParserInput {
        input: InputSource::new_at(content, content_start),
        state: parser_input.state.clone(),
    };
    let previous_block_mode = child_input
        .state
        .replace_block_mode(BlockMode::NestedDocument);
    child_input
        .state
        .increase_depth()
        .map_err(|e| e.into_context_error())?;
    let parsed_content = parse_document_input(&mut child_input);
    child_input.state.decrease_depth();
    child_input.state.replace_block_mode(previous_block_mode);
    parser_input.state = child_input.state;
    let end = raw_content.close_end;

    Ok(Element::BlockQuote(BlockQuoteElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: raw_content.close_start,
            end,
        },
        marker_spans: vec![],
        parameters: parameters.unwrap_or_default(),
        children: parsed_content,
    }))
}
