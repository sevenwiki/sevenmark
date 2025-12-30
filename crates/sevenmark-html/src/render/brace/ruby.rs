//! Ruby element rendering

use maud::{Markup, html};
use sevenmark_parser::ast::RubyElement;

use crate::classes;
use crate::context::RenderContext;
use crate::render::{render_elements, utils};

pub fn render(e: &RubyElement, ctx: &mut RenderContext) -> Markup {
    ctx.enter_suppress_soft_breaks();
    let base = render_elements(&e.content, ctx);
    ctx.exit_suppress_soft_breaks();

    let ruby_text = utils::get_param(&e.parameters, "ruby").unwrap_or_default();
    let style = utils::build_style(&e.parameters);

    html! {
        ruby class=(classes::RUBY) style=[style] {
            (base)
            rt { (ruby_text) }
        }
    }
}
