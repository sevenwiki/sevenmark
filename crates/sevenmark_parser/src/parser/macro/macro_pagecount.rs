use crate::parser::ParserInput;
use sevenmark_ast::{Element, PageCountElement, Span};
use winnow::Result;
use winnow::combinator::alt;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_until};

pub fn macro_pagecount_parser(parser_input: &mut ParserInput) -> Result<Element> {
    let start = parser_input.current_token_start();

    let namespace = alt((pagecount_with_ns, pagecount_bare)).parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(Element::PageCount(PageCountElement {
        span: Span { start, end },
        namespace,
    }))
}

fn pagecount_with_ns(parser_input: &mut ParserInput) -> Result<Option<String>> {
    literal("[pagecount(").parse_next(parser_input)?;
    let ns = take_until(0.., ")]").parse_next(parser_input)?;
    literal(")]").parse_next(parser_input)?;
    Ok(Some(ns.to_string()))
}

fn pagecount_bare(parser_input: &mut ParserInput) -> Result<Option<String>> {
    literal("[pagecount]").parse_next(parser_input)?;
    Ok(None)
}
