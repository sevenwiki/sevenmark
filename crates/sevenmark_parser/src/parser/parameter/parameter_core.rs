use crate::parser::ParserInput;
use crate::parser::parameter::parameter_content::parameter_content_parser;
use sevenmark_ast::{Parameter, Parameters, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt, preceded, repeat, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

/// Parse a single parameter in the format #key="value" (spaces around = allowed)
/// The value part is optional - if not provided, an empty Vec is used
fn parameter_parser(parser_input: &mut ParserInput) -> Result<(String, Parameter)> {
    // Consume leading whitespace before recording start so the span begins at '#',
    // not at any preceding newline (which would make the token invisible on its line).
    multispace0.parse_next(parser_input)?;
    let start = parser_input.current_token_start();

    // Parse: #key, optional ="value"
    let (_, key, value_opt): (_, &str, _) = (
        literal('#'),
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '-'),
        opt(preceded(
            delimited(multispace0, literal('='), multispace0),
            delimited(literal('"'), parameter_content_parser, literal('"')),
        )),
    )
        .parse_next(parser_input)?;

    // Record end before consuming trailing whitespace so the span is tight.
    let end = parser_input.previous_token_end();
    multispace0.parse_next(parser_input)?;

    let key_string = key.to_string();
    let value = value_opt.unwrap_or_else(Vec::new);

    let parameter = Parameter {
        span: Span { start, end },
        key: key_string.clone(),
        value,
    };

    Ok((key_string, parameter))
}

/// Parse multiple parameters and collect them into a Parameters map
/// Terminated by an optional "||" followed by whitespace
/// Returns a Parameters map where keys are parameter names and values are SevenMarkElement vectors
pub fn parameter_core_parser(parser_input: &mut ParserInput) -> Result<Parameters> {
    terminated(
        // Parse one or more parameters and directly collect into Parameters
        repeat(1.., parameter_parser)
            .map(|pairs: Vec<_>| pairs.into_iter().collect::<Parameters>()),
        // End marker: optional "||" followed by whitespace
        preceded(opt(literal("||")), multispace0),
    )
    .parse_next(parser_input)
}
