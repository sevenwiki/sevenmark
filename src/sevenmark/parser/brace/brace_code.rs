use crate::sevenmark::ast::{Location, SevenMarkElement};
use crate::sevenmark::parser::brace::code::code_content_parser;
use crate::sevenmark::parser::parameter::parameter_core_parser;
use crate::sevenmark::parser::utils::with_depth;
use crate::sevenmark::{CodeElement, ParserInput};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_code_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#code"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| with_depth(input, code_content_parser),
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    let language = parameters
        .as_ref()
        .and_then(|p| p.get("lang"))
        .cloned()
        .unwrap_or_else(Vec::new);

    Ok(SevenMarkElement::CodeElement(CodeElement {
        location: Location { start, end },
        language,
        content: parsed_content,
    }))
}
