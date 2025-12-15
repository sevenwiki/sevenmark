//! Element renderers organized by syntax type

pub mod brace;
pub mod bracket;
pub mod r#macro;
pub mod markdown;
pub mod text;
pub mod utils;

use crate::context::RenderContext;
use maud::{Markup, html};
use sevenmark_parser::ast::SevenMarkElement;

/// Render a single element to HTML
pub fn render_element(elem: &SevenMarkElement, ctx: &mut RenderContext) -> Markup {
    match elem {
        // Text elements
        SevenMarkElement::Text(t) => text::render_text(t),
        SevenMarkElement::Escape(e) => text::render_escape(e),
        SevenMarkElement::LiteralElement(l) => text::render_literal(l, ctx),

        // Markdown inline styles
        SevenMarkElement::Bold(s) => markdown::render_bold(s, ctx),
        SevenMarkElement::Italic(s) => markdown::render_italic(s, ctx),
        SevenMarkElement::BoldItalic(s) => markdown::render_bold_italic(s, ctx),
        SevenMarkElement::Strikethrough(s) => markdown::render_strikethrough(s, ctx),
        SevenMarkElement::Underline(s) => markdown::render_underline(s, ctx),
        SevenMarkElement::Superscript(s) => markdown::render_superscript(s, ctx),
        SevenMarkElement::Subscript(s) => markdown::render_subscript(s, ctx),

        // Markdown block elements
        SevenMarkElement::Header(h) => markdown::render_header(h, ctx),
        SevenMarkElement::HLine => markdown::render_hline(),
        SevenMarkElement::NewLine => markdown::render_newline(),

        // Brace elements {{{...}}}
        SevenMarkElement::StyledElement(s) => brace::render_styled(s, ctx),
        SevenMarkElement::BlockQuoteElement(b) => brace::render_blockquote(b, ctx),
        SevenMarkElement::CodeElement(c) => brace::render_code(c, ctx),
        SevenMarkElement::FoldElement(f) => brace::render_fold(f, ctx),
        SevenMarkElement::TableElement(t) => brace::render_table(t, ctx),
        SevenMarkElement::ListElement(l) => brace::render_list(l, ctx),
        SevenMarkElement::RubyElement(r) => brace::render_ruby(r, ctx),
        SevenMarkElement::TeXElement(t) => brace::render_tex(t),
        SevenMarkElement::FootnoteElement(f) => brace::render_footnote(f, ctx),

        // Bracket elements [[...]]
        SevenMarkElement::MediaElement(m) => bracket::render_media(m, ctx),

        // Macros
        SevenMarkElement::TimeNow => r#macro::render_time_now(ctx),
        SevenMarkElement::Age(a) => r#macro::render_age(a, ctx),
        SevenMarkElement::FootNote => html! {}, // FootNote marker without content - skip

        // Ignored elements (processed in transform or metadata)
        SevenMarkElement::Comment(_)
        | SevenMarkElement::Error(_)
        | SevenMarkElement::Null
        | SevenMarkElement::DefineElement(_)
        | SevenMarkElement::Variable(_)
        | SevenMarkElement::IfElement(_)
        | SevenMarkElement::Include(_)
        | SevenMarkElement::Category(_)
        | SevenMarkElement::Redirect(_) => html! {},
    }
}
