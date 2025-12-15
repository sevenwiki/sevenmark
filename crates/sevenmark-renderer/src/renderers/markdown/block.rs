//! Block element renderers (markdown)

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::Header;

/// Render header (h1-h6)
pub fn render_header(header: &Header, ctx: &mut RenderContext) -> Markup {
    let id = format!("s-{}", header.section_index);
    let content = render_elements(&header.content, ctx);

    match header.level {
        1 => html! { h1 id=(id) { (content) } },
        2 => html! { h2 id=(id) { (content) } },
        3 => html! { h3 id=(id) { (content) } },
        4 => html! { h4 id=(id) { (content) } },
        5 => html! { h5 id=(id) { (content) } },
        _ => html! { h6 id=(id) { (content) } },
    }
}

/// Render horizontal line
pub fn render_hline() -> Markup {
    html! { hr; }
}

/// Render newline (line break)
pub fn render_newline() -> Markup {
    html! { br; }
}
