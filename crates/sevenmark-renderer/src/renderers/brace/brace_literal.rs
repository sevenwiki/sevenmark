use maud::Markup;
use sevenmark_parser::ast::LiteralElement;
use crate::context::RenderContext;
use crate::render_elements;

/// Render literal element (render children)
pub fn render_brace_literal(elem: &LiteralElement, ctx: &mut RenderContext) -> Markup {
    render_elements(&elem.content, ctx)
}
