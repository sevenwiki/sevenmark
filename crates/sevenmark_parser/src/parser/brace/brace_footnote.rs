use crate::parser::ParserInput;
use crate::parser::element::element_parser;
use crate::parser::parameter::parameter_core_parser;
use crate::parser::utils::with_depth_and_trim;
use sevenmark_ast::{Element, FootnoteElement, Span};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

/// Parse footnote elements enclosed in {{{#fn }}}
pub fn brace_footnote_parser(parser_input: &mut ParserInput) -> Result<Element> {
    if parser_input.state.inside_footnote {
        return Err(winnow::error::ContextError::new());
    }
    let start = parser_input.current_token_start();

    literal("{{{#fn").parse_next(parser_input)?;
    let open_end = parser_input.previous_token_end();

    let parameters = opt(parameter_core_parser).parse_next(parser_input)?;
    multispace0.parse_next(parser_input)?;

    parser_input.state.set_footnote_context();
    let parsed_content = with_depth_and_trim(parser_input, element_parser);
    parser_input.state.unset_footnote_context();
    let parsed_content = parsed_content?;

    multispace0.parse_next(parser_input)?;
    let close_start = parser_input.current_token_start();
    literal("}}}").parse_next(parser_input)?;
    let end = parser_input.previous_token_end();

    let footnote_index = parser_input.state.next_footnote_index();

    Ok(Element::Footnote(FootnoteElement {
        span: Span { start, end },
        open_span: Span {
            start,
            end: open_end,
        },
        close_span: Span {
            start: close_start,
            end,
        },
        footnote_index,
        parameters: parameters.unwrap_or_default(),
        children: parsed_content,
    }))
}
