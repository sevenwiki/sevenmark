//! Code block renderer

use crate::context::RenderContext;
use crate::render_elements;
use crate::renderers::utils::get_param_string;
use maud::{Markup, html};
use sevenmark_parser::ast::CodeElement;

/// Render code block element
pub fn render_code(elem: &CodeElement, ctx: &mut RenderContext) -> Markup {
    let lang = get_param_string(&elem.parameters, "lang");
    let content = render_elements(&elem.content, ctx);

    html! {
        pre {
            @if let Some(lang) = lang {
                code class=(format!("language-{}", lang)) {
                    (content)
                }
            } @else {
                code {
                    (content)
                }
            }
        }
    }
}
