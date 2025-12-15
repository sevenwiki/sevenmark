//! Inline style renderers (markdown text styles)

use crate::context::RenderContext;
use crate::render_elements;
use maud::{Markup, html};
use sevenmark_parser::ast::TextStyle;

/// Render bold text
pub fn render_bold(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        strong { (render_elements(&style.content, ctx)) }
    }
}

/// Render italic text
pub fn render_italic(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        em { (render_elements(&style.content, ctx)) }
    }
}

/// Render bold + italic text
pub fn render_bold_italic(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        strong {
            em { (render_elements(&style.content, ctx)) }
        }
    }
}

/// Render strikethrough text
pub fn render_strikethrough(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        del { (render_elements(&style.content, ctx)) }
    }
}

/// Render underline text
pub fn render_underline(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        u { (render_elements(&style.content, ctx)) }
    }
}

/// Render superscript text
pub fn render_superscript(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        sup { (render_elements(&style.content, ctx)) }
    }
}

/// Render subscript text
pub fn render_subscript(style: &TextStyle, ctx: &mut RenderContext) -> Markup {
    html! {
        sub { (render_elements(&style.content, ctx)) }
    }
}
