use crate::core::parse_document;
use crate::parser::ParserInput;
use sevenmark_ast::{Element, ListContentItem, ListElement, ListItemElement, Span};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, eof, repeat, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_while};

pub fn markdown_list_parser(parser_input: &mut ParserInput) -> Result<Element> {
    if parser_input.state.current_depth() > 0 {
        return Err(winnow::error::ContextError::new());
    }

    let current_pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(current_pos) {
        return Err(winnow::error::ContextError::new());
    }

    let start = current_pos;

    let items: Vec<ListItemElement> =
        repeat(1.., list_item_line).parse_next(parser_input)?;

    let end = parser_input.previous_token_end();

    Ok(Element::List(ListElement {
        span: Span { start, end },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        kind: String::new(),
        parameters: Default::default(),
        children: items.into_iter().map(ListContentItem::Item).collect(),
    }))
}

fn list_item_line(parser_input: &mut ParserInput) -> Result<ListItemElement> {
    let pos = parser_input.current_token_start();
    if !parser_input.state.is_at_line_start(pos) {
        return Err(winnow::error::ContextError::new());
    }

    let item_start = pos;

    literal("-").parse_next(parser_input)?;
    literal(" ").parse_next(parser_input)?;

    let content: &str = terminated(
        take_while(0.., |c: char| c != '\n'),
        alt((line_ending, eof)),
    )
    .parse_next(parser_input)?;

    let item_end = parser_input.previous_token_end();

    let children = parse_document(content);

    Ok(ListItemElement {
        span: Span {
            start: item_start,
            end: item_end,
        },
        open_span: Span::synthesized(),
        close_span: Span::synthesized(),
        parameters: Default::default(),
        children,
    })
}