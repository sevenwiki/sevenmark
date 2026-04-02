use scraper::{Html, Selector};

use crate::{RenderConfig, render_document};
use sevenmark_parser::core::parse_document;

pub(crate) fn render_html(input: &str) -> String {
    let ast = parse_document(input);
    render_document(&ast, &RenderConfig::default())
}

pub(crate) fn parse_fragment(html: &str) -> Html {
    Html::parse_fragment(html)
}

pub(crate) fn selector(value: &str) -> Selector {
    Selector::parse(value).expect("valid selector")
}
