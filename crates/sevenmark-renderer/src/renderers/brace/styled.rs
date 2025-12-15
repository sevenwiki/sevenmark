//! Styled element renderer

use crate::context::RenderContext;
use crate::render_elements;
use crate::renderers::utils::build_style_string;
use maud::{Markup, html};
use sevenmark_parser::ast::StyledElement;

/// Render styled element with inline styles
///
/// Always uses `<span>` - display mode is controlled by CSS via `#block` / `#inline` parameters.
/// This simplifies the rendering logic while giving users explicit control.
pub fn render_styled(elem: &StyledElement, ctx: &mut RenderContext) -> Markup {
    let style = build_style_string(&elem.parameters);
    let content = render_elements(&elem.content, ctx);

    html! {
        @if let Some(ref style) = style {
            span class="sm-styled" style=(style) {
                (content)
            }
        } @else {
            span class="sm-styled" {
                (content)
            }
        }
    }
}
