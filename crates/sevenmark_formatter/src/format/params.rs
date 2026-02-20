use pretty::{Arena, DocAllocator, DocBuilder};
use sevenmark_ast::Parameters;

use super::element::format_elements;

/// Format parameters as ` #key="value"` pairs (space-prefixed).
/// Flag parameters (empty value) are rendered as ` #key`.
pub fn format_params<'a>(a: &'a Arena<'a>, params: &Parameters) -> DocBuilder<'a, Arena<'a>> {
    format_params_inner(a, params, true, false)
}

/// Format parameters as `#key="value"` pairs without leading space.
pub fn format_params_tight<'a>(a: &'a Arena<'a>, params: &Parameters) -> DocBuilder<'a, Arena<'a>> {
    format_params_inner(a, params, false, false)
}

/// Format parameters with leading space and trailing `||` separator.
/// For `{{{#tag #key="val" ||` style.
pub fn format_params_block<'a>(a: &'a Arena<'a>, params: &Parameters) -> DocBuilder<'a, Arena<'a>> {
    format_params_inner(a, params, true, true)
}

/// Format parameters without leading space, with trailing `||` separator.
/// For `[[#key="val" ||` style (cells, rows, list items).
pub fn format_params_block_tight<'a>(
    a: &'a Arena<'a>,
    params: &Parameters,
) -> DocBuilder<'a, Arena<'a>> {
    format_params_inner(a, params, false, true)
}

fn format_params_inner<'a>(
    a: &'a Arena<'a>,
    params: &Parameters,
    leading_space: bool,
    trailing_separator: bool,
) -> DocBuilder<'a, Arena<'a>> {
    if params.is_empty() {
        return a.nil();
    }

    let mut doc = a.nil();
    let mut first = true;
    for param in params.values() {
        if first && !leading_space {
            doc = doc.append(a.text("#"));
            first = false;
        } else {
            doc = doc.append(a.text(" #"));
        }
        doc = doc.append(a.text(param.key.clone()));
        if !param.value.is_empty() {
            doc = doc
                .append(a.text("=\""))
                .append(format_elements(a, &param.value))
                .append(a.text("\""));
        }
    }
    if trailing_separator {
        doc = doc.append(a.text(" ||"));
    }
    doc
}
