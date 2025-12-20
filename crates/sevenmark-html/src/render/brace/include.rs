//! Include element rendering

use maud::Markup;
use sevenmark_parser::ast::IncludeElement;

use crate::context::RenderContext;
use crate::render::render_elements;

pub fn render(e: &IncludeElement, ctx: &mut RenderContext) -> Markup {
    // Include content is already resolved by transform - just render it
    render_elements(&e.content, ctx)
}
