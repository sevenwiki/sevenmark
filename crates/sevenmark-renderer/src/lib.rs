//! HTML renderer for SevenMark AST
//!
//! Renders processed SevenMark AST to HTML using maud.

pub mod context;
pub mod renderers;

use context::RenderContext;
use maud::{Markup, html};
use sevenmark_parser::ast::SevenMarkElement;

/// Render AST elements to HTML string
pub fn render(ast: &[SevenMarkElement]) -> String {
    let mut ctx = RenderContext::new();
    render_with_context(ast, &mut ctx)
}

/// Render AST elements to HTML string with custom context
pub fn render_with_context(ast: &[SevenMarkElement], ctx: &mut RenderContext) -> String {
    let body = render_elements(ast, ctx);
    let footnotes = renderers::brace::render_footnotes_section(ctx);

    html! {
        (body)
        (footnotes)
    }
    .into_string()
}

/// Render multiple elements
pub fn render_elements(elements: &[SevenMarkElement], ctx: &mut RenderContext) -> Markup {
    html! {
        @for elem in elements {
            (renderers::render_element(elem, ctx))
        }
    }
}
