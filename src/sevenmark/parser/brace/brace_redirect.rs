use crate::sevenmark::ast::{Location, SevenMarkElement};
use crate::sevenmark::parser::brace::redirect::redirect_content_parser;
use crate::sevenmark::parser::utils::with_depth;
use crate::sevenmark::{ParserInput, RedirectElement};
use winnow::ascii::multispace0;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use winnow::Result;

pub fn brace_redirect_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (_, parsed_content) = delimited(
        literal("{{{#redirect"),
        (multispace0, |input: &mut ParserInput| {
            with_depth(input, redirect_content_parser)
        }),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Redirect(RedirectElement {
        location: Location { start, end },
        content: parsed_content,
    }))
}
