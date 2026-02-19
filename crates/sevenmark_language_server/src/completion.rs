use sevenmark_ast::Element;
use sevenmark_utils::extract_plain_text;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionItemKind, CompletionTextEdit, InsertTextFormat, Position, Range,
    TextEdit,
};

use crate::ast_walk::visit_elements;
use crate::document::DocumentState;

/// Returns completion items based on the cursor context.
pub fn get_completions(
    state: &DocumentState,
    position: Position,
    byte_offset: usize,
) -> Vec<CompletionItem> {
    let prefix = &state.text[..byte_offset];

    // `[var(` → suggest defined variable names
    if prefix.ends_with("[var(") {
        return variable_completions(state);
    }

    // `{{{#` → suggest brace element keywords
    if prefix.ends_with("{{{#") {
        return brace_keyword_completions(position);
    }

    // `[` at macro position → suggest macro names
    if prefix.ends_with('[') && !prefix.ends_with("[[") {
        return macro_completions(position);
    }

    Vec::new()
}

/// Collects all defined variable names in the document.
fn variable_completions(state: &DocumentState) -> Vec<CompletionItem> {
    let mut names = Vec::new();
    visit_elements(&state.elements, &mut |element| {
        if let Element::Define(d) = element {
            if let Some(name_param) = d.parameters.get("name") {
                let name = extract_plain_text(&name_param.value);
                if !name.is_empty() && !names.contains(&name) {
                    names.push(name);
                }
            }
        }
    });

    names
        .into_iter()
        .map(|name| CompletionItem {
            label: name,
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        })
        .collect()
}

/// Brace element keyword completions after `{{{#`.
fn brace_keyword_completions(pos: Position) -> Vec<CompletionItem> {
    let keywords = [
        ("code", "code #lang=\"$1\"\n$0\n}}}", "Code block"),
        ("table", "table\n$0\n}}}", "Table"),
        ("list", "list\n$0\n}}}", "List"),
        ("fold", "fold\n$0\n}}}", "Fold (collapsible)"),
        ("style", "style #style=\"$1\"\n$0\n}}}", "Styled block"),
        ("blockquote", "blockquote\n$0\n}}}", "Block quote"),
        ("define", "define #name=\"$1\" #value=\"$2\"}}}", "Variable definition"),
        ("if", "if $1 ::\n$0\n}}}", "Conditional block"),
        ("include", "include $0}}}", "Document inclusion"),
        ("category", "category $0}}}", "Category"),
        ("redirect", "redirect $0}}}", "Redirect"),
        ("ruby", "ruby #ruby=\"$1\" $0}}}", "Ruby annotation"),
        ("footnote", "fn $0}}}", "Footnote"),
        ("literal", "literal\n$0\n}}}", "Literal output"),
    ];

    // The snippet replaces the `{{{#` trigger, so adjust the range
    let start = Position::new(pos.line, pos.character.saturating_sub(4));
    let range = Range::new(start, pos);

    keywords
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range,
                new_text: format!("{{{{{{#{snippet}"),
            })),
            ..Default::default()
        })
        .collect()
}

/// Macro completions after `[`.
fn macro_completions(pos: Position) -> Vec<CompletionItem> {
    let macros = [
        ("var", "var($1)]", "Variable reference"),
        ("br", "br]", "Line break"),
        ("null", "null]", "Null (no output)"),
        ("fn", "fn]", "Footnote reference"),
        ("now", "now]", "Current time"),
        ("age", "age($1)]", "Age calculation"),
    ];

    let start = Position::new(pos.line, pos.character.saturating_sub(1));
    let range = Range::new(start, pos);

    macros
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                range,
                new_text: format!("[{snippet}"),
            })),
            ..Default::default()
        })
        .collect()
}
