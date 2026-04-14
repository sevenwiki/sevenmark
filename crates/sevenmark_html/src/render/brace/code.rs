//! Code block rendering

use maud::{Markup, html};
use sevenmark_ast::{Parameters, Span};

use crate::classes;
use crate::context::RenderContext;
use crate::render::utils;

pub fn render(span: &Span, parameters: &Parameters, value: &str, ctx: &RenderContext) -> Markup {
    let lang = utils::get_param(parameters, "lang");
    let style = utils::build_style(parameters);
    let merged_class = utils::merge_class(classes::CODE, parameters);
    let (dk, dark_tag) = utils::dark_style_parts(utils::build_dark_style(parameters));
    html! {
        (dark_tag)
        pre
            class=(merged_class)
            data-start=[ctx.span_start(span)]
            data-end=[ctx.span_end(span)]
            style=[style]
            data-dk=[dk]
        {
            code data-lang=[lang] { (value) }
        }
    }
}
