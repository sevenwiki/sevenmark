use sevenmark_ast::Element;
use tower_lsp_server::ls_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range};

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;

/// Returns hover information for the element at the given byte offset.
pub fn get_hover(state: &DocumentState, byte_offset: usize) -> Option<Hover> {
    // Find the most specific (deepest/smallest) element containing the offset.
    // We store (content, span_start, span_end, span_len) as owned values.
    let mut best: Option<(String, usize, usize, usize)> = None;

    visit_elements(&state.elements, &mut |element| {
        let span = element.span();
        if span.start <= byte_offset && byte_offset < span.end {
            let len = span.end - span.start;
            if let Some(content) = hover_content(element) {
                if best.as_ref().is_none_or(|(_, _, _, best_len)| len < *best_len) {
                    best = Some((content, span.start, span.end, len));
                }
            }
        }
    });

    let (content, start_offset, end_offset, _) = best?;
    let start = state.line_index.byte_offset_to_position(&state.text, start_offset);
    let end = state.line_index.byte_offset_to_position(&state.text, end_offset);

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: Some(Range::new(
            Position::new(start.0, start.1),
            Position::new(end.0, end.1),
        )),
    })
}

/// Generates markdown hover text for an element.
fn hover_content(element: &Element) -> Option<String> {
    let text = match element {
        Element::Header(h) => format!("**Header** (level {})", h.level),
        Element::Bold(_) => "**Bold**".to_string(),
        Element::Italic(_) => "**Italic**".to_string(),
        Element::Strikethrough(_) => "**Strikethrough**".to_string(),
        Element::Underline(_) => "**Underline**".to_string(),
        Element::Superscript(_) => "**Superscript**".to_string(),
        Element::Subscript(_) => "**Subscript**".to_string(),
        Element::Code(c) => {
            let lang = c
                .parameters
                .get("lang")
                .map(|p| sevenmark_utils::extract_plain_text(&p.value));
            match lang {
                Some(l) if !l.is_empty() => format!("**Code** (lang: `{l}`)"),
                _ => "**Code**".to_string(),
            }
        }
        Element::TeX(t) => {
            if t.is_block {
                "**TeX** (block)".to_string()
            } else {
                "**TeX** (inline)".to_string()
            }
        }
        Element::Variable(v) => format!("**Variable**: `{}`", v.name),
        Element::Define(_) => "**Define** — variable definition".to_string(),
        Element::Include(_) => "**Include** — document inclusion".to_string(),
        Element::Category(_) => "**Category**".to_string(),
        Element::Redirect(_) => "**Redirect**".to_string(),
        Element::Media(_) => "**Media** `[[...]]`".to_string(),
        Element::ExternalMedia(e) => format!("**External Media**: `{}`", e.provider),
        Element::Table(_) => "**Table**".to_string(),
        Element::List(l) => format!("**List** ({})", l.kind),
        Element::Fold(_) => "**Fold** — collapsible block".to_string(),
        Element::BlockQuote(_) => "**Block Quote**".to_string(),
        Element::Styled(_) => "**Styled** — custom styled block".to_string(),
        Element::Literal(_) => "**Literal** — raw output".to_string(),
        Element::Ruby(_) => "**Ruby** — ruby annotation".to_string(),
        Element::Footnote(_) => "**Footnote**".to_string(),
        Element::If(_) => "**If** — conditional block".to_string(),
        Element::Mention(m) => {
            let kind = match &m.kind {
                sevenmark_ast::MentionType::User => "user",
                sevenmark_ast::MentionType::Discussion => "discussion",
            };
            format!("**Mention** ({kind}): `{}`", m.id)
        }
        Element::Error(e) => {
            let preview: String = e.value.chars().take(40).collect();
            format!("**Error**: `{preview}`")
        }
        // Leaf nodes that don't need hover
        Element::Text(_)
        | Element::Comment(_)
        | Element::Escape(_)
        | Element::Null(_)
        | Element::FootnoteRef(_)
        | Element::TimeNow(_)
        | Element::Age(_)
        | Element::SoftBreak(_)
        | Element::HardBreak(_)
        | Element::HLine(_) => return None,
    };
    Some(text)
}