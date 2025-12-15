//! Ruby (furigana) element renderer

use crate::context::RenderContext;
use crate::render_elements;
use crate::renderers::utils::get_param_string;
use maud::{Markup, html};
use sevenmark_parser::ast::RubyElement;

/// Render ruby element for furigana/annotations
pub fn render_ruby(elem: &RubyElement, ctx: &mut RenderContext) -> Markup {
    let rt = get_param_string(&elem.parameters, "rt").unwrap_or_default();

    html! {
        ruby class="sm-ruby" {
            (render_elements(&elem.content, ctx))
            rp { "(" }
            rt { (rt) }
            rp { ")" }
        }
    }
}
