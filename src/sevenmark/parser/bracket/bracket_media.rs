use crate::sevenmark::ast::{Location, MediaElement, SevenMarkElement};
use crate::sevenmark::parser::element::element_parser;
use crate::sevenmark::parser::parameter::parameter_core_parser;
use crate::sevenmark::parser::utils::with_depth;
use crate::sevenmark::ParserInput;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use winnow::Result;

/// Parse media elements enclosed in [[ ]] with parameters
pub fn bracket_media_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    // MediaElement 중첩 방지
    if parser_input.state.inside_media_element {
        return Err(winnow::error::ContextError::new());
    }

    let start = parser_input.input.current_token_start();

    let (parameters, parsed_content) = delimited(
        literal("[["),
        (opt(parameter_core_parser), |input: &mut ParserInput| {
            opt(|inner_input: &mut ParserInput| {
                inner_input.state.set_media_element_context();
                let result = with_depth(inner_input, element_parser);
                inner_input.state.unset_media_element_context();
                result
            })
            .parse_next(input)
        }),
        literal("]]"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();
    
    Ok(SevenMarkElement::MediaElement(MediaElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content.unwrap_or_default(),
    }))
}
