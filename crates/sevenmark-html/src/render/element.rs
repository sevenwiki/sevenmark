//! Element-level rendering

use maud::{Markup, html};
use sevenmark_parser::ast::SevenMarkElement;

use super::{brace, bracket, r#macro, markdown, mention, text};
use crate::context::RenderContext;

/// Render multiple elements
pub fn render_elements(elements: &[SevenMarkElement], ctx: &mut RenderContext) -> Markup {
    html! {
        @for el in elements {
            (render_element(el, ctx))
        }
    }
}

/// Render a single element (dispatch to specific renderers)
pub fn render_element(el: &SevenMarkElement, ctx: &mut RenderContext) -> Markup {
    match el {
        // Text elements
        SevenMarkElement::Text(e) => text::text::render(e),
        SevenMarkElement::Escape(e) => text::escape::render(e),
        SevenMarkElement::Comment(_) => html! {},
        SevenMarkElement::Error(e) => text::error::render(e),

        // Markdown text styles
        SevenMarkElement::Bold(e) => markdown::bold::render(e, ctx),
        SevenMarkElement::Italic(e) => markdown::italic::render(e, ctx),
        SevenMarkElement::Strikethrough(e) => markdown::strikethrough::render(e, ctx),
        SevenMarkElement::Underline(e) => markdown::underline::render(e, ctx),
        SevenMarkElement::Superscript(e) => markdown::superscript::render(e, ctx),
        SevenMarkElement::Subscript(e) => markdown::subscript::render(e, ctx),

        // Header (handled by section tree, should not appear in content)
        SevenMarkElement::Header(_) => html! {},

        // Block elements
        SevenMarkElement::BlockQuoteElement(e) => brace::blockquote::render(e, ctx),
        SevenMarkElement::LiteralElement(e) => brace::literal::render(e, ctx),
        SevenMarkElement::StyledElement(e) => brace::styled::render(e, ctx),
        SevenMarkElement::FoldElement(e) => brace::fold::render(e, ctx),
        SevenMarkElement::RubyElement(e) => brace::ruby::render(e, ctx),
        SevenMarkElement::CodeElement(e) => brace::code::render(e),
        SevenMarkElement::TeXElement(e) => brace::tex::render(e),

        // Container elements
        SevenMarkElement::ListElement(e) => brace::list::render(e, ctx),
        SevenMarkElement::TableElement(e) => brace::table::render(e, ctx),

        // Media
        SevenMarkElement::MediaElement(e) => bracket::media::render(e, ctx),

        // Footnotes
        SevenMarkElement::FootnoteElement(e) => brace::footnote::render(e, ctx),
        SevenMarkElement::FootNote => r#macro::footnote::render(ctx),

        // Line breaks
        SevenMarkElement::SoftBreak => r#macro::newline::render_soft_break(ctx),
        SevenMarkElement::HardBreak => r#macro::newline::render_hard_break(),

        // Macros
        SevenMarkElement::HLine => r#macro::hline::render(),
        SevenMarkElement::TimeNow => r#macro::timenow::render(),
        SevenMarkElement::Age(e) => r#macro::age::render(e),

        // Wiki elements (metadata, not rendered visually)
        SevenMarkElement::Category(_) => html! {},
        SevenMarkElement::Redirect(_) => html! {},
        SevenMarkElement::Include(e) => brace::include::render(e, ctx),
        SevenMarkElement::DefineElement(_) => html! {},
        SevenMarkElement::Variable(e) => text::variable::render(e),
        SevenMarkElement::IfElement(_) => html! {},

        // Mentions
        SevenMarkElement::Mention(e) => mention::mention::render(e),

        SevenMarkElement::Null => html! {},
    }
}
