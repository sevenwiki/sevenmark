use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Element;

use crate::FormatConfig;

use super::brace;
use super::bracket;
use super::macros;
use super::markdown;
use super::text;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SoftBreakPolicy {
    Preserve,
    Suppress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrailingSoftBreakPolicy {
    Preserve,
    Drop,
    AsHardBreak,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormatContext {
    pub soft_break_policy: SoftBreakPolicy,
    pub trailing_soft_break_policy: TrailingSoftBreakPolicy,
}

impl Default for FormatContext {
    fn default() -> Self {
        Self {
            soft_break_policy: SoftBreakPolicy::Preserve,
            trailing_soft_break_policy: TrailingSoftBreakPolicy::Preserve,
        }
    }
}

impl FormatContext {
    pub fn suppress_soft_breaks(self) -> Self {
        Self {
            soft_break_policy: SoftBreakPolicy::Suppress,
            ..self
        }
    }

    pub fn with_trailing_soft_break_policy(self, policy: TrailingSoftBreakPolicy) -> Self {
        Self {
            trailing_soft_break_policy: policy,
            ..self
        }
    }
}

/// Format multiple elements, concatenating their output.
pub fn format_elements<'a>(
    a: &'a Arena<'a>,
    elements: &[Element],
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    format_elements_with_context(a, elements, config, FormatContext::default())
}

pub fn format_elements_with_context<'a>(
    a: &'a Arena<'a>,
    elements: &[Element],
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    let mut doc = a.nil();
    let trailing_soft_break_start = elements
        .len()
        .saturating_sub(count_trailing_soft_breaks(elements));

    for (index, el) in elements.iter().enumerate() {
        if matches!(el, Element::SoftBreak(_)) && context.soft_break_policy == SoftBreakPolicy::Suppress {
            continue;
        }

        if index >= trailing_soft_break_start && matches!(el, Element::SoftBreak(_)) {
            match context.trailing_soft_break_policy {
                TrailingSoftBreakPolicy::Preserve => {}
                TrailingSoftBreakPolicy::Drop => continue,
                TrailingSoftBreakPolicy::AsHardBreak => {
                    doc = doc.append(macros::format_hard_break(a));
                    continue;
                }
            }
        }

        doc = doc.append(format_element_with_context(a, el, config, context));
        if needs_terminating_newline(el) {
            doc = doc.append(a.hardline());
        }
    }
    doc
}

fn count_trailing_soft_breaks(elements: &[Element]) -> usize {
    elements
        .iter()
        .rev()
        .take_while(|el| matches!(el, Element::SoftBreak(_)))
        .count()
}

/// Some parsers consume trailing line breaks/whitespace and do not emit SoftBreak.
/// We must emit a separator to keep the AST parse-stable across format -> parse.
fn needs_terminating_newline(el: &Element) -> bool {
    match el {
        Element::Header(_)
        | Element::HLine(_)
        | Element::Define(_)
        | Element::Include(_)
        | Element::Category(_) => true,
        Element::Comment(e) => !e.value.contains('\n'),
        _ => false,
    }
}

/// Format a single element by dispatching to the appropriate formatter.
pub fn format_element<'a>(
    a: &'a Arena<'a>,
    el: &Element,
    config: &FormatConfig,
) -> DocBuilder<'a, Arena<'a>> {
    format_element_with_context(a, el, config, FormatContext::default())
}

pub fn format_element_with_context<'a>(
    a: &'a Arena<'a>,
    el: &Element,
    config: &FormatConfig,
    context: FormatContext,
) -> DocBuilder<'a, Arena<'a>> {
    match el {
        // Basic text elements
        Element::Text(e) => text::format_text(a, e),
        Element::Escape(e) => text::format_escape(a, e),
        Element::Error(e) => text::format_error(a, e),
        Element::Comment(e) => text::format_comment(a, e),
        Element::Mention(e) => text::format_mention(a, e),

        // Line elements
        Element::SoftBreak(_) => match context.soft_break_policy {
            SoftBreakPolicy::Preserve => a.hardline(),
            SoftBreakPolicy::Suppress => a.nil(),
        },
        Element::HardBreak(_) => macros::format_hard_break(a),
        Element::Clear(_) => macros::format_clear(a),
        Element::HLine(_) => macros::format_hline(a),

        // Macros
        Element::Null(_) => macros::format_null(a),
        Element::FootnoteRef(_) => macros::format_footnote_ref(a),
        Element::TimeNow(_) => macros::format_time_now(a),
        Element::Date(_) => macros::format_date(a),
        Element::DateTime(_) => macros::format_datetime(a),
        Element::Dday(e) => macros::format_dday(a, e),
        Element::PageCount(e) => macros::format_pagecount(a, e),
        Element::Age(e) => macros::format_age(a, e),
        Element::Variable(e) => macros::format_variable(a, e),
        Element::Anchor(e) => macros::format_anchor(a, e),
        Element::Toc(_) => macros::format_toc(a),

        // Markdown text styles
        Element::Bold(e) => markdown::bold::format_bold(a, &e.children, config, context),
        Element::Italic(e) => markdown::italic::format_italic(a, &e.children, config, context),
        Element::Strikethrough(e) => {
            markdown::strikethrough::format_strikethrough(a, &e.children, config, context)
        }
        Element::Underline(e) => markdown::underline::format_underline(a, &e.children, config, context),
        Element::Superscript(e) => {
            markdown::superscript::format_superscript(a, &e.children, config, context)
        }
        Element::Subscript(e) => markdown::subscript::format_subscript(a, &e.children, config, context),
        Element::Header(e) => markdown::header::format_header(a, e, config, context),

        // Bracket elements
        Element::Media(e) => bracket::media::format_media(a, e, config, context),
        Element::ExternalMedia(e) => bracket::external_media::format_external_media(a, e, config),

        // Brace block elements
        Element::Literal(e) => brace::literal::format_literal(a, e, config, context),
        Element::Define(e) => brace::define::format_define(a, e, config),
        Element::Styled(e) => brace::styled::format_styled(a, e, config, context),
        Element::Table(e) => brace::table::format_table(a, e, config, context),
        Element::List(e) => brace::list::format_list(a, e, config, context),
        Element::Fold(e) => brace::fold::format_fold(a, e, config, context),
        Element::BlockQuote(e) => brace::blockquote::format_blockquote(a, e, config, context),
        Element::Ruby(e) => brace::ruby::format_ruby(a, e, config, context),
        Element::Footnote(e) => brace::footnote::format_footnote(a, e, config, context),
        Element::Code(e) => brace::code::format_code(a, e, config),
        Element::TeX(e) => brace::tex::format_tex(a, e),
        Element::Css(e) => brace::css::format_css(a, e, config),
        Element::Include(e) => brace::include::format_include(a, e, config, context),
        Element::Category(e) => brace::category::format_category(a, e, config, context),
        Element::Redirect(e) => brace::redirect::format_redirect(a, e, config, context),

        // Conditional
        Element::If(e) => brace::conditional::format_if(a, e, config, context),
    }
}
