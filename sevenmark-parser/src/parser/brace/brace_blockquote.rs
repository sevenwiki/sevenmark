use crate::ast::{BlockQuoteElement, Location, SevenMarkElement};
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

pub fn brace_blockquote_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#quote"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| with_depth(input, element_parser),
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::BlockQuoteElement(BlockQuoteElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    }))
}
