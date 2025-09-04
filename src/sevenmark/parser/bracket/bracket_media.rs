use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{Location, MediaElement, SevenMarkElement};
use crate::sevenmark::parser::element::element_parser;
use crate::sevenmark::parser::parameter::parameter_core_parser;
use crate::sevenmark::parser::utils::with_depth;
use winnow::Result;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse media elements enclosed in [[ ]] with parameters
pub fn bracket_media_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (parameters, parsed_content) = delimited(
        literal("[["),
        (opt(parameter_core_parser), |input: &mut ParserInput| {
            opt(|inner_input: &mut ParserInput| with_depth(inner_input, element_parser))
                .parse_next(input)
        }),
        literal("]]"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    let url = parameters
        .as_ref()
        .and_then(|p| p.get("url"))
        .map(|param| param.value.clone())
        .unwrap_or_default();
    let file = parameters
        .as_ref()
        .and_then(|p| p.get("file"))
        .map(|param| param.value.clone())
        .unwrap_or_default();
    let display_text = parsed_content.unwrap_or_default();

    // If no parameters, treat content as URL (hyperlink behavior)
    let (final_url, final_display_text) = if parameters.is_none() && !display_text.is_empty() {
        (display_text.clone(), Vec::new())
    } else {
        (url, display_text)
    };

    Ok(SevenMarkElement::MediaElement(MediaElement {
        location: Location { start, end },
        file,
        url: final_url,
        display_text: final_display_text,
    }))
}
