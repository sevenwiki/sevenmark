//! Text element renderers

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::{EscapeElement, LiteralElement, TextElement};

/// Render plain text (auto-escaped by maud)
pub fn render_text(elem: &TextElement) -> Markup {
    html! { (elem.content) }
}

/// Render escaped character (already escaped in source)
pub fn render_escape(elem: &EscapeElement) -> Markup {
    html! { (elem.content) }
}

/// Render literal element (render children)
pub fn render_literal(elem: &LiteralElement, ctx: &mut RenderContext) -> Markup {
    render_elements(&elem.content, ctx)
}