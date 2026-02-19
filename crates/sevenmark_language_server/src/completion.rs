use sevenmark_ast::Element;
use tower_lsp_server::ls_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

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

    // `[[` → suggest bracket element keywords (media, external media)
    if prefix.ends_with("[[") {
        return bracket_completions(position);
    }

    // `[` at macro position → suggest macro names
    if prefix.ends_with('[') {
        return macro_completions(position);
    }

    Vec::new()
}

/// Collects all defined variable names in the document.
fn variable_completions(state: &DocumentState) -> Vec<CompletionItem> {
    let mut names = Vec::new();
    visit_elements(&state.elements, &mut |element| {
        if let Element::Define(d) = element {
            for name in d.parameters.keys() {
                if !names.contains(name) {
                    names.push(name.clone());
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
fn brace_keyword_completions(_pos: Position) -> Vec<CompletionItem> {
    let keywords = [
        ("code", "code #lang=\"$1\"\n$0\n}}}", "Code block"),
        ("table", "table\n$0\n}}}", "Table"),
        ("list", "list\n$0\n}}}", "List"),
        ("fold", "fold\n$0\n}}}", "Fold (collapsible)"),
        ("style", "style #style=\"$1\"\n$0\n}}}", "Styled block"),
        ("blockquote", "blockquote\n$0\n}}}", "Block quote"),
        ("define", "define #$1=\"$2\"}}}", "Variable definition"),
        ("if", "if $1 ::\n$0\n}}}", "Conditional block"),
        ("include", "include $0}}}", "Document inclusion"),
        ("category", "category $0}}}", "Category"),
        ("redirect", "redirect $0}}}", "Redirect"),
        ("ruby", "ruby #ruby=\"$1\" $0}}}", "Ruby annotation"),
        ("footnote", "fn $0}}}", "Footnote"),
    ];

    keywords
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(snippet.to_string()),
            ..Default::default()
        })
        .collect()
}

/// Bracket element completions after `[[`.
fn bracket_completions(_pos: Position) -> Vec<CompletionItem> {
    let items = [
        ("media", "#file=\"$1\" $0]]", "Media (internal file)"),
        ("link", "$1]]", "Wiki link"),
        ("youtube", "#youtube #id=\"$1\"]]", "YouTube embed"),
        ("vimeo", "#vimeo #id=\"$1\"]]", "Vimeo embed"),
        ("nicovideo", "#nicovideo #id=\"$1\"]]", "NicoVideo embed"),
        ("spotify", "#spotify #id=\"$1\"]]", "Spotify embed"),
        ("discord", "#discord #id=\"$1\"]]", "Discord embed"),
    ];

    items
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(snippet.to_string()),
            ..Default::default()
        })
        .collect()
}

/// Macro completions after `[`.
fn macro_completions(_pos: Position) -> Vec<CompletionItem> {
    let macros = [
        ("var", "var($1)]", "Variable reference"),
        ("br", "br]", "Line break"),
        ("null", "null]", "Null (no output)"),
        ("fn", "fn]", "Footnote reference"),
        ("now", "now]", "Current time"),
        ("age", "age($1)]", "Age calculation"),
    ];

    macros
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(snippet.to_string()),
            ..Default::default()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp_server::ls_types::Position;

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    #[test]
    fn var_prefix_with_define_suggests_variable() {
        let text = "{{{#define #myvar=\"v\"}}}[var(";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty(), "expected variable completions");
        assert!(completions.iter().any(|c| c.label == "myvar"));
    }

    #[test]
    fn brace_prefix_suggests_keywords() {
        let text = "{{{#";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty());
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"code"), "expected 'code' in {labels:?}");
        assert!(labels.contains(&"table"), "expected 'table' in {labels:?}");
        assert!(labels.contains(&"list"), "expected 'list' in {labels:?}");
        assert!(
            !labels.contains(&"literal"),
            "did not expect invalid 'literal' brace keyword completion in {labels:?}"
        );
    }

    #[test]
    fn bracket_prefix_suggests_macros() {
        let text = "hello [";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty(), "expected macro completions");
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"var"), "expected 'var' in {labels:?}");
        assert!(labels.contains(&"br"), "expected 'br' in {labels:?}");
    }

    #[test]
    fn double_bracket_suggests_media() {
        let text = "hello [[";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty(), "expected bracket completions");
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"media"), "expected 'media' in {labels:?}");
        assert!(
            labels.contains(&"youtube"),
            "expected 'youtube' in {labels:?}"
        );
        assert!(labels.contains(&"link"), "expected 'link' in {labels:?}");
    }

    #[test]
    fn no_trigger_empty_completions() {
        let text = "hello world";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(completions.is_empty());
    }
}
