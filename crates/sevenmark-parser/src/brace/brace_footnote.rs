use super::super::element::element_parser;
use crate::ast::{FootnoteElement, SevenMarkElement};
use crate::{Location, ParserInput};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse footnote elements enclosed in {{{#fn }}}
pub fn brace_footnote_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_footnote {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.input.current_token_start();

    let parsed_content = delimited(
        literal("{{{#fn"),
        |input: &mut ParserInput| {
            let mut inner_input = input.clone();
            inner_input
                .state
                .increase_depth()
                .map_err(|e| e.into_context_error())?;
            inner_input.state.set_footnote_context();
            let result = element_parser(&mut inner_input);
            inner_input.state.unset_footnote_context();
            inner_input
                .state
                .decrease_depth()
                .map_err(|e| e.into_context_error())?;
            *input = inner_input;
            result
        },
        literal("}}}"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::FootnoteElement(FootnoteElement {
        location: Location { start, end },
        content: parsed_content,
    }))
}
