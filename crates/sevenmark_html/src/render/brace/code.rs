//! Code block rendering

use maud::{Markup, html};
use sevenmark_parser::ast::{Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

pub fn render(span: &Span, parameters: &Parameters, value: &str, ctx: &RenderContext) -> Markup {
    let lang = utils::get_param(parameters, "lang");
    html! {
        pre
            class=(classes::CODE)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
        {
            code data-lang=[lang] { (value) }
        }
    }
}
