//! TeX/LaTeX element renderer

use maud::{Markup, html};
use sevenmark_parser::ast::TeXElement;

/// Render TeX element
/// Note: Actual TeX rendering should be done client-side with KaTeX/MathJax
pub fn render_tex(elem: &TeXElement) -> Markup {
    if elem.is_block {
        html! {
            div class="sm-tex sm-tex-block" {
                (elem.content)
            }
        }
    } else {
        html! {
            span class="sm-tex sm-tex-inline" {
                (elem.content)
            }
        }
    }
}
