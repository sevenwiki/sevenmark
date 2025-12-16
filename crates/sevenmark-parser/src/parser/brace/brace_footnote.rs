use crate::ast::{FootnoteElement, Location, SevenMarkElement};
use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::with_depth;
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse footnote elements enclosed in {{{#fn }}}
pub fn brace_footnote_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_footnote {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#fn"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| {
                input.state.set_footnote_context();
                let result = with_depth(input, element_parser);
                input.state.unset_footnote_context();
                result
            },
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::FootnoteElement(FootnoteElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    }))
}
