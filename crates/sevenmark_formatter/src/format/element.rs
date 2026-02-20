use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Element;

use super::brace;
use super::bracket;
use super::macros;
use super::markdown;
use super::text;

/// Format multiple elements, concatenating their output.
pub fn format_elements<'a>(a: &'a Arena<'a>, elements: &[Element]) -> DocBuilder<'a, Arena<'a>> {
    let mut doc = a.nil();
    for el in elements {
        doc = doc.append(format_element(a, el));
    }
    doc
}

/// Format a single element by dispatching to the appropriate formatter.
pub fn format_element<'a>(a: &'a Arena<'a>, el: &Element) -> DocBuilder<'a, Arena<'a>> {
    match el {
        // Basic text elements
        Element::Text(e) => text::format_text(a, e),
        Element::Escape(e) => text::format_escape(a, e),
        Element::Error(e) => text::format_error(a, e),
        Element::Comment(e) => text::format_comment(a, e),
        Element::Mention(e) => text::format_mention(a, e),

        // Line elements
        Element::SoftBreak(_) => a.hardline(),
        Element::HardBreak(_) => macros::format_hard_break(a),
        Element::HLine(_) => macros::format_hline(a),

        // Macros
        Element::Null(_) => macros::format_null(a),
        Element::FootnoteRef(_) => macros::format_footnote_ref(a),
        Element::TimeNow(_) => macros::format_time_now(a),
        Element::Age(e) => macros::format_age(a, e),
        Element::Variable(e) => macros::format_variable(a, e),

        // Markdown text styles
        Element::Bold(e) => markdown::bold::format_bold(a, &e.children),
        Element::Italic(e) => markdown::italic::format_italic(a, &e.children),
        Element::Strikethrough(e) => markdown::strikethrough::format_strikethrough(a, &e.children),
        Element::Underline(e) => markdown::underline::format_underline(a, &e.children),
        Element::Superscript(e) => markdown::superscript::format_superscript(a, &e.children),
        Element::Subscript(e) => markdown::subscript::format_subscript(a, &e.children),
        Element::Header(e) => markdown::header::format_header(a, e),

        // Bracket elements
        Element::Media(e) => bracket::media::format_media(a, e),
        Element::ExternalMedia(e) => bracket::external_media::format_external_media(a, e),

        // Brace block elements
        Element::Literal(e) => brace::literal::format_literal(a, e),
        Element::Define(e) => brace::define::format_define(a, e),
        Element::Styled(e) => brace::styled::format_styled(a, e),
        Element::Table(e) => brace::table::format_table(a, e),
        Element::List(e) => brace::list::format_list(a, e),
        Element::Fold(e) => brace::fold::format_fold(a, e),
        Element::BlockQuote(e) => brace::blockquote::format_blockquote(a, e),
        Element::Ruby(e) => brace::ruby::format_ruby(a, e),
        Element::Footnote(e) => brace::footnote::format_footnote(a, e),
        Element::Code(e) => brace::code::format_code(a, e),
        Element::TeX(e) => brace::tex::format_tex(a, e),
        Element::Include(e) => brace::include::format_include(a, e),
        Element::Category(e) => brace::category::format_category(a, e),
        Element::Redirect(e) => brace::redirect::format_redirect(a, e),

        // Conditional
        Element::If(e) => brace::conditional::format_if(a, e),
    }
}
