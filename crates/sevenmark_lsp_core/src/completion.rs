use std::collections::BTreeSet;

use ls_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};
use sevenmark_ast::Element;

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

    // `#` inside bracket/brace element → suggest element-specific parameters
    if prefix.ends_with('#') {
        if let Some(items) = parameter_completions(prefix) {
            return items;
        }
    }

    // `[` at macro position → suggest macro names
    if prefix.ends_with('[') {
        return macro_completions(position);
    }

    Vec::new()
}

/// Collects all defined variable names in the document.
fn variable_completions(state: &DocumentState) -> Vec<CompletionItem> {
    let mut names = BTreeSet::new();
    visit_elements(&state.elements, &mut |element| {
        if let Element::Define(d) = element {
            for name in d.parameters.keys() {
                names.insert(name.clone());
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
        ("file", "#file=\"$1\" $0]]", "File / image media"),
        ("document", "#document=\"$1\" $0]]", "Document link"),
        ("category", "#category=\"$1\"]]", "Category link"),
        ("user", "#user=\"$1\"]]", "User link"),
        ("url", "#url=\"$1\" $0]]", "External URL link"),
        ("link", "$1]]", "Wiki link"),
        ("youtube", "#youtube #id=\"$1\"]]", "YouTube embed"),
        ("vimeo", "#vimeo #id=\"$1\"]]", "Vimeo embed"),
        ("nicovideo", "#nicovideo #id=\"$1\"]]", "NicoVideo embed"),
        ("spotify", "#spotify $0]]", "Spotify embed"),
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
// ── Parameter completions ────────────────────────────────────────────

/// Detects bracket element context and returns the element keyword.
/// Given `...[[#youtube #id="abc" #`, returns `Some("youtube")`.
fn detect_bracket_element(prefix: &str) -> Option<&str> {
    let bracket_pos = prefix.rfind("[[")?;
    let after = &prefix[bracket_pos + 2..];
    if after.contains("]]") {
        return None;
    }
    let after = after.strip_prefix('#')?;
    let end = after
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    Some(&after[..end])
}

/// Detects brace element context and returns the element keyword.
/// Given `...{{{#code #lang="rust" #`, returns `Some("code")`.
fn detect_brace_element(prefix: &str) -> Option<&str> {
    let brace_pos = prefix.rfind("{{{#")?;
    let after = &prefix[brace_pos + 4..];
    if after.contains("}}}") {
        return None;
    }
    let end = after
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(after.len());
    if end == 0 {
        return None;
    }
    Some(&after[..end])
}

/// Tries to return parameter completions for the current element context.
fn parameter_completions(prefix: &str) -> Option<Vec<CompletionItem>> {
    if let Some(element) = detect_bracket_element(prefix) {
        let params = bracket_param_defs(element);
        if !params.is_empty() {
            return Some(make_param_completions(params));
        }
    }
    if let Some(element) = detect_brace_element(prefix) {
        let params = brace_param_defs(element);
        if !params.is_empty() {
            return Some(make_param_completions(params));
        }
    }
    None
}

/// Parameter definition: (name, description, is_flag).
/// Flag parameters insert just the name; value parameters insert `name="$1"`.
type ParamDef = (&'static str, &'static str, bool);

fn bracket_param_defs(element: &str) -> &'static [ParamDef] {
    match element {
        "youtube" => &[
            ("id", "Video ID", false),
            ("playlist", "Playlist ID", false),
            ("width", "Player width", false),
            ("height", "Player height", false),
            ("start", "Start time (seconds)", false),
            ("end", "End time (seconds)", false),
            ("autoplay", "Auto-play", true),
            ("loop", "Loop playback", true),
            ("mute", "Muted", true),
            ("nocontrols", "Hide controls", true),
        ],
        "vimeo" => &[
            ("id", "Video ID", false),
            ("h", "Privacy hash", false),
            ("width", "Player width", false),
            ("height", "Player height", false),
            ("autoplay", "Auto-play", true),
            ("loop", "Loop playback", true),
            ("mute", "Muted", true),
            ("color", "Player accent color", false),
            ("dnt", "Do-not-track", true),
        ],
        "nicovideo" => &[
            ("id", "Video ID", false),
            ("width", "Player width", false),
            ("height", "Player height", false),
            ("from", "Start time (seconds)", false),
            ("autoplay", "Auto-play", true),
        ],
        "spotify" => &[
            ("track", "Track ID", false),
            ("album", "Album ID", false),
            ("playlist", "Playlist ID", false),
            ("artist", "Artist ID", false),
            ("episode", "Episode ID", false),
            ("show", "Show / Podcast ID", false),
            ("width", "Player width", false),
            ("height", "Player height", false),
            ("dark", "Dark theme", true),
            ("compact", "Compact layout", true),
        ],
        "discord" => &[
            ("id", "Widget / Server ID", false),
            ("width", "Widget width", false),
            ("height", "Widget height", false),
            ("dark", "Dark theme", true),
        ],
        "file" => &[
            ("file", "File path", false),
            ("style", "Display style", false),
        ],
        "document" => &[
            ("document", "Document path", false),
            ("style", "Display style", false),
        ],
        "url" => &[
            ("url", "External URL", false),
            ("style", "Display style", false),
        ],
        _ => &[],
    }
}

fn brace_param_defs(element: &str) -> &'static [ParamDef] {
    match element {
        "code" => &[("lang", "Programming language", false)],
        "style" => &[("style", "CSS style", false)],
        "ruby" => &[("ruby", "Ruby text annotation", false)],
        _ => &[],
    }
}

fn make_param_completions(params: &[ParamDef]) -> Vec<CompletionItem> {
    params
        .iter()
        .map(|(name, detail, is_flag)| {
            let snippet = if *is_flag {
                name.to_string()
            } else {
                format!("{name}=\"$1\"")
            };
            CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(detail.to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text: Some(snippet),
                ..Default::default()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ls_types::Position;

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
    }

    #[test]
    fn brace_prefix_does_not_suggest_literal_keyword() {
        let text = "{{{#";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
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
    fn double_bracket_suggests_bracket_elements() {
        let text = "hello [[";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty(), "expected bracket completions");
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"file"), "expected 'file' in {labels:?}");
        assert!(
            labels.contains(&"document"),
            "expected 'document' in {labels:?}"
        );
        assert!(
            labels.contains(&"category"),
            "expected 'category' in {labels:?}"
        );
        assert!(labels.contains(&"url"), "expected 'url' in {labels:?}");
        assert!(labels.contains(&"link"), "expected 'link' in {labels:?}");
        assert!(
            labels.contains(&"youtube"),
            "expected 'youtube' in {labels:?}"
        );
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

    #[test]
    fn youtube_param_completions() {
        let text = "[[#youtube #";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(
            !completions.is_empty(),
            "expected youtube param completions"
        );
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"id"), "expected 'id' in {labels:?}");
        assert!(labels.contains(&"width"), "expected 'width' in {labels:?}");
        assert!(
            labels.contains(&"autoplay"),
            "expected 'autoplay' in {labels:?}"
        );
    }

    #[test]
    fn spotify_param_completions() {
        let text = "[[#spotify #";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        let labels: Vec<_> = completions.iter().map(|c| c.label.as_str()).collect();
        assert!(labels.contains(&"track"), "expected 'track' in {labels:?}");
        assert!(labels.contains(&"album"), "expected 'album' in {labels:?}");
        assert!(!labels.contains(&"id"), "spotify should not have 'id'");
    }

    #[test]
    fn brace_code_param_completions() {
        let text = "{{{#code #";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(!completions.is_empty(), "expected code param completions");
        assert!(completions.iter().any(|c| c.label == "lang"));
    }

    #[test]
    fn flag_param_has_no_equals() {
        let text = "[[#youtube #";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        let autoplay = completions.iter().find(|c| c.label == "autoplay").unwrap();
        assert_eq!(
            autoplay.insert_text.as_deref(),
            Some("autoplay"),
            "flag params should not include =\"$1\""
        );
        let id = completions.iter().find(|c| c.label == "id").unwrap();
        assert_eq!(
            id.insert_text.as_deref(),
            Some("id=\"$1\""),
            "value params should include =\"$1\""
        );
    }

    #[test]
    fn closed_bracket_no_param_completions() {
        let text = "[[#youtube #id=\"abc\"]] #";
        let state = make_state(text);
        let byte_offset = text.len();
        let pos = Position::new(0, byte_offset as u32);
        let completions = get_completions(&state, pos, byte_offset);
        assert!(
            completions.is_empty(),
            "should not suggest params after closed bracket"
        );
    }
}
