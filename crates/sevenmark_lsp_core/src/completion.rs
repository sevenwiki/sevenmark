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

    let ctx = context_and_bracket_depth(prefix);
    // ctx = Some((keyword, opens, closes)) where depth = opens.saturating_sub(closes)

    // `{{{#` → context-aware brace keywords
    if prefix.ends_with("{{{#") {
        return brace_hash_completions(ctx);
    }

    // `[[#` → context-aware
    if prefix.ends_with("[[#") {
        return bracket_hash_completions_ctx(ctx, position);
    }

    // `[[` → context-aware
    if prefix.ends_with("[[") {
        return bracket_completions_ctx(ctx, position);
    }

    // `#` inside element → element-specific parameter suggestions
    if prefix.ends_with('#') {
        if let Some(items) = parameter_completions(prefix, ctx) {
            return items;
        }
    }

    // `[` → macro names
    if prefix.ends_with('[') {
        return macro_completions(position);
    }

    Vec::new()
}

// ── Core context detector ─────────────────────────────────────────

/// Walks the prefix tracking `{{{#keyword` opens and `}}}` closes.
/// Returns `(innermost_keyword, bracket_depth_from_that_context)`.
///
/// bracket depth is counted as unclosed `[[` since the innermost `{{{#` open.
fn context_and_bracket_depth(prefix: &str) -> Option<(&str, usize)> {
    // stack of (brace_pos, kw_end) — positions into `prefix`
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < prefix.len() {
        if prefix[i..].starts_with("{{{#") {
            let kw_start = i + 4;
            let kw_end = prefix[kw_start..]
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .map(|e| kw_start + e)
                .unwrap_or(prefix.len());
            if kw_end > kw_start {
                stack.push((i, kw_end));
            }
            i += 4;
        } else if prefix[i..].starts_with("}}}") {
            stack.pop();
            i += 3;
        } else {
            // Advance by one Unicode scalar, not one byte —
            // slicing a &str at a non-char-boundary panics.
            i += prefix[i..].chars().next().map_or(1, |c| c.len_utf8());
        }
    }

    let &(brace_pos, kw_end) = stack.last()?;
    let keyword = &prefix[brace_pos + 4..kw_end];

    let after = &prefix[brace_pos..];
    let opens = after.matches("[[").count();
    let closes = after.matches("]]").count();
    let depth = opens.saturating_sub(closes);

    Some((keyword, depth))
}

// ── Dispatch functions ────────────────────────────────────────────

/// `{{{#` trigger — which brace elements are valid here?
fn brace_hash_completions(ctx: Option<(&str, usize)>) -> Vec<CompletionItem> {
    let structural = match ctx {
        // table: row level (d=0) and cell structural level (d=1) only allow {{{#if
        Some(("table", d)) if d < 2 => true,
        // list: item structural level (d=0) only allows {{{#if
        Some(("list", d)) if d < 1 => true,
        _ => false,
    };
    if structural {
        brace_conditional_completions()
    } else {
        brace_keyword_completions()
    }
}

/// `[[` trigger — structural contexts only.
///
/// At content levels, bare `[[text]]` (no `#` params) renders as empty HTML,
/// so no completions are offered. Only structural positions within `{{{#table}}`,
/// `{{{#list}}}` and `{{{#fold}}}` get context-aware snippets; everything else
/// returns nothing (use `[[#` for actual links/media).
fn bracket_completions_ctx(ctx: Option<(&str, usize)>, _pos: Position) -> Vec<CompletionItem> {
    match ctx {
        // ── table ──────────────────────────────────────────────
        Some(("table", 1)) => table_row_completions(),
        Some(("table", 2)) => table_cell_completions(),

        // ── list ───────────────────────────────────────────────
        Some(("list", 1)) => list_item_completions(),

        // ── fold ───────────────────────────────────────────────
        Some(("fold", 1)) => fold_section_completions(),

        // ── content level or unknown → no completions ──────────
        _ => Vec::new(),
    }
}

/// `[[#` trigger — keyword after `#` or element-specific params.
fn bracket_hash_completions_ctx(ctx: Option<(&str, usize)>, pos: Position) -> Vec<CompletionItem> {
    match ctx {
        // ── table ──────────────────────────────────────────────
        // depth 1: row level — rows have no keyword params
        Some(("table", 1)) => Vec::new(),
        // depth 2: cell level — show x/y params
        Some(("table", 2)) => make_param_completions(table_cell_param_defs()),
        // depth ≥ 3: inside cell content — generic media keywords
        Some(("table", _)) => generic_bracket_hash_completions(pos),

        // ── list ───────────────────────────────────────────────
        // depth 1: item level — items have no keyword params
        Some(("list", 1)) => Vec::new(),
        // depth ≥ 2: inside item content — generic
        Some(("list", _)) => generic_bracket_hash_completions(pos),

        // ── fold ───────────────────────────────────────────────
        // depth 1/2: fold section level — no known section params
        Some(("fold", 1)) | Some(("fold", 2)) => Vec::new(),
        // depth ≥ 3: inside section content — generic
        Some(("fold", _)) => generic_bracket_hash_completions(pos),

        // ── top-level or unknown context ───────────────────────
        _ => generic_bracket_hash_completions(pos),
    }
}

// ── Variable completions ──────────────────────────────────────────

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

// ── Brace keyword completions ─────────────────────────────────────

fn brace_keyword_completions() -> Vec<CompletionItem> {
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

/// Only `{{{#if` — valid at structural levels of table / list.
fn brace_conditional_completions() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "if".to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some("Conditional block".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some("if $1 ::\n$0\n}}}".to_string()),
        ..Default::default()
    }]
}

// ── Generic bracket completions (top-level / content) ─────────────

/// `[[#` at top-level / content — keyword already typed.
fn generic_bracket_hash_completions(_pos: Position) -> Vec<CompletionItem> {
    let items = [
        ("file", "file=\"$1\" $0]]", "File / image media"),
        ("document", "document=\"$1\" $0]]", "Document link"),
        ("category", "category=\"$1\"]]", "Category link"),
        ("user", "user=\"$1\"]]", "User link"),
        ("url", "url=\"$1\" $0]]", "External URL link"),
        ("youtube", "youtube #id=\"$1\"]]", "YouTube embed"),
        ("vimeo", "vimeo #id=\"$1\"]]", "Vimeo embed"),
        ("nicovideo", "nicovideo #id=\"$1\"]]", "NicoVideo embed"),
        ("spotify", "spotify $0]]", "Spotify embed"),
        ("discord", "discord #id=\"$1\"]]", "Discord embed"),
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

// ── Table-specific completions ────────────────────────────────────

fn table_row_completions() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "row".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Table row".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some("$0]]".to_string()),
        ..Default::default()
    }]
}

fn table_cell_completions() -> Vec<CompletionItem> {
    let items = [
        ("cell", "$0]]", "Table cell"),
        (
            "cell (x,y)",
            "#x=\"$1\" #y=\"$2\" $0]]",
            "Table cell with position",
        ),
    ];
    items
        .into_iter()
        .map(|(label, snippet, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some(detail.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text: Some(snippet.to_string()),
            ..Default::default()
        })
        .collect()
}

// ── List-specific completions ─────────────────────────────────────

fn list_item_completions() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "item".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("List item".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some("$0]]".to_string()),
        ..Default::default()
    }]
}

// ── Fold-specific completions ─────────────────────────────────────

/// `[[` at fold section level — just provide a closing template.
/// Fold has two sections: `[[title]] [[content]]`, neither uses a keyword.
fn fold_section_completions() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "section".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Fold section (title or content)".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text: Some("$0]]".to_string()),
        ..Default::default()
    }]
}

// ── Macro completions ─────────────────────────────────────────────

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

// ── Parameter completions ─────────────────────────────────────────

/// Detects bracket element context (`[[#keyword`) and returns the keyword.
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

/// Detects brace element context (`{{{#keyword`) and returns the keyword.
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

fn parameter_completions(prefix: &str, ctx: Option<(&str, usize)>) -> Option<Vec<CompletionItem>> {
    // Bracket element params (e.g. [[#youtube #id=...)
    if let Some(kw) = detect_bracket_element(prefix) {
        let params = bracket_param_defs(kw);
        if !params.is_empty() {
            return Some(make_param_completions(params));
        }
    }
    // Brace element params (e.g. {{{#code #lang=...)
    if let Some(kw) = detect_brace_element(prefix) {
        let params = brace_param_defs(kw);
        if !params.is_empty() {
            return Some(make_param_completions(params));
        }
    }
    // Table cell params: `#` inside an unclosed `[[` at cell depth
    if let Some(("table", 2)) = ctx {
        if let Some(bracket_pos) = prefix.rfind("[[") {
            let after = &prefix[bracket_pos + 2..];
            if !after.contains("]]") {
                return Some(make_param_completions(table_cell_param_defs()));
            }
        }
    }
    None
}

// ── Parameter definitions ─────────────────────────────────────────

/// `(name, description, is_flag)` — flags insert just the name, values insert `name="$1"`.
type ParamDef = (&'static str, &'static str, bool);

fn table_cell_param_defs() -> &'static [ParamDef] {
    &[
        ("x", "Column position / span", false),
        ("y", "Row position / span", false),
    ]
}

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

// ── Tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use ls_types::Position;

    fn pos() -> Position {
        Position::new(0, 0)
    }

    fn make_state(text: &str) -> DocumentState {
        DocumentState::new(text.to_string())
    }

    fn completions(text: &str) -> Vec<CompletionItem> {
        let state = make_state(text);
        let byte_offset = text.len();
        get_completions(&state, pos(), byte_offset)
    }

    fn labels(items: &[CompletionItem]) -> Vec<&str> {
        items.iter().map(|c| c.label.as_str()).collect()
    }

    // ── context_and_bracket_depth ─────────────────────────────────

    #[test]
    fn context_top_level_is_none() {
        assert!(context_and_bracket_depth("hello world").is_none());
    }

    #[test]
    fn context_multibyte_chars_do_not_panic() {
        // Korean / emoji in prefix must not cause a char-boundary panic
        assert!(context_and_bracket_depth("한글 테스트").is_none());
        assert_eq!(
            context_and_bracket_depth("{{{#table\n  한글 [["),
            Some(("table", 1))
        );
        assert!(context_and_bracket_depth("🚀🚀🚀").is_none());
    }

    #[test]
    fn context_table_depth_0() {
        assert_eq!(
            context_and_bracket_depth("{{{#table\n  "),
            Some(("table", 0))
        );
    }

    #[test]
    fn context_table_depth_1_row() {
        assert_eq!(
            context_and_bracket_depth("{{{#table\n  [["),
            Some(("table", 1))
        );
    }

    #[test]
    fn context_table_depth_2_cell() {
        assert_eq!(
            context_and_bracket_depth("{{{#table\n  [[ [["),
            Some(("table", 2))
        );
    }

    #[test]
    fn context_table_closed_row_next_row() {
        // Completed row balances out; next [[ is depth 1 again
        assert_eq!(
            context_and_bracket_depth("{{{#table\n  [[ [[c]] ]]\n  [["),
            Some(("table", 1))
        );
    }

    #[test]
    fn context_list_in_table_cell() {
        // List inside a table cell → innermost is "list"
        let prefix = "{{{#table\n  [[ [[\n    {{{#list\n      [[";
        assert_eq!(context_and_bracket_depth(prefix), Some(("list", 1)));
    }

    #[test]
    fn context_table_in_list_item() {
        // Table inside a list item → innermost is "table"
        let prefix = "{{{#list\n  [[\n    {{{#table\n      [[";
        assert_eq!(context_and_bracket_depth(prefix), Some(("table", 1)));
    }

    #[test]
    fn context_after_closed_inner_returns_outer() {
        // After the inner {{{#list}}} closes, outer table is the context
        let prefix = "{{{#table\n  [[ [[\n    {{{#list [[item]] }}}\n    [[";
        assert_eq!(context_and_bracket_depth(prefix), Some(("table", 3)));
    }

    // ── Top-level completions ─────────────────────────────────────

    #[test]
    fn var_prefix_suggests_defined_variable() {
        let c = completions("{{{#define #myvar=\"v\"}}}[var(");
        assert!(c.iter().any(|c| c.label == "myvar"));
    }

    #[test]
    fn top_level_brace_hash_suggests_all_keywords() {
        let c = completions("{{{#");
        let l = labels(&c);
        assert!(l.contains(&"code"));
        assert!(l.contains(&"table"));
        assert!(l.contains(&"list"));
        assert!(l.contains(&"if"));
        assert!(!l.contains(&"literal"));
    }

    #[test]
    fn top_level_bracket_no_completions() {
        // [[  alone has no completions: bare [[text]] renders as empty HTML.
        // Meaningful links all require [[#  (document, file, url, etc.)
        let c = completions("hello [[");
        assert!(c.is_empty(), "bare [[ should produce no completions: {c:?}");
    }

    #[test]
    fn top_level_bracket_hash_suggests_generic_no_link() {
        let c = completions("hello [[#");
        let l = labels(&c);
        assert!(l.contains(&"file"));
        assert!(l.contains(&"youtube"));
        assert!(!l.contains(&"link")); // link has no # prefix
    }

    #[test]
    fn top_level_bracket_hash_snippet_has_no_leading_hash() {
        let c = completions("hello [[#");
        let file = c.iter().find(|c| c.label == "file").unwrap();
        let snippet = file.insert_text.as_deref().unwrap();
        assert!(
            !snippet.starts_with('#'),
            "snippet should not start with #: {snippet}"
        );
    }

    // ── Table context ─────────────────────────────────────────────

    #[test]
    fn table_brace_hash_structural_suggests_only_if() {
        // depth 0 — between rows
        let c = completions("{{{#table\n  {{{#");
        let l = labels(&c);
        assert!(l.contains(&"if"));
        assert!(!l.contains(&"code"));
        assert!(!l.contains(&"table"));
    }

    #[test]
    fn table_brace_hash_inside_row_suggests_only_if() {
        // depth 1 — inside a row (between cells)
        let c = completions("{{{#table\n  [[ {{{#");
        let l = labels(&c);
        assert!(l.contains(&"if"));
        assert!(!l.contains(&"code"));
    }

    #[test]
    fn table_brace_hash_inside_cell_content_suggests_all() {
        // depth 2 — inside cell content
        let c = completions("{{{#table\n  [[ [[ {{{#");
        let l = labels(&c);
        assert!(l.contains(&"code"));
        assert!(l.contains(&"list"));
        assert!(l.contains(&"if"));
    }

    #[test]
    fn table_bracket_row_level() {
        let c = completions("{{{#table\n  [[");
        let l = labels(&c);
        assert!(l.contains(&"row"));
        assert!(!l.contains(&"file"));
        assert!(!l.contains(&"youtube"));
    }

    #[test]
    fn table_bracket_cell_level() {
        let c = completions("{{{#table\n  [[ [[");
        let l = labels(&c);
        assert!(l.contains(&"cell"));
        assert!(!l.contains(&"file"));
    }

    #[test]
    fn table_bracket_inside_cell_content_no_completions() {
        // depth ≥ 3: inside cell content, bare [[ → no completions (needs [[#)
        let c = completions("{{{#table\n  [[ [[ [[");
        assert!(
            c.is_empty(),
            "bare [[ inside cell content should produce no completions: {c:?}"
        );
    }

    #[test]
    fn table_bracket_hash_row_level_empty() {
        let c = completions("{{{#table\n  [[#");
        assert!(c.is_empty(), "row level [[# should have no completions");
    }

    #[test]
    fn table_bracket_hash_cell_level_shows_xy() {
        let c = completions("{{{#table\n  [[ [[#");
        let l = labels(&c);
        assert!(l.contains(&"x"));
        assert!(l.contains(&"y"));
        assert!(!l.contains(&"youtube"));
    }

    #[test]
    fn table_bracket_hash_inside_cell_content_is_generic() {
        let c = completions("{{{#table\n  [[ [[ [[#");
        let l = labels(&c);
        assert!(l.contains(&"youtube"));
        assert!(!l.contains(&"x"));
    }

    #[test]
    fn table_param_hash_shows_xy_for_cell() {
        let c = completions("{{{#table\n  [[ [[ #");
        let l = labels(&c);
        assert!(l.contains(&"x"));
        assert!(l.contains(&"y"));
    }

    #[test]
    fn table_outside_closed_bracket_no_completions() {
        // After closed table, bare [[ at top level → no completions
        let c = completions("{{{#table\n  [[ [[c]] ]]\n}}}\n\n[[");
        assert!(
            c.is_empty(),
            "bare [[ after closed table should produce no completions: {c:?}"
        );
    }

    // ── List context ──────────────────────────────────────────────

    #[test]
    fn list_brace_hash_structural_suggests_only_if() {
        let c = completions("{{{#list\n  {{{#");
        let l = labels(&c);
        assert!(l.contains(&"if"));
        assert!(!l.contains(&"code"));
    }

    #[test]
    fn list_brace_hash_inside_item_suggests_all() {
        let c = completions("{{{#list\n  [[ {{{#");
        let l = labels(&c);
        assert!(l.contains(&"code"));
        assert!(l.contains(&"if"));
    }

    #[test]
    fn list_bracket_item_level() {
        let c = completions("{{{#list\n  [[");
        let l = labels(&c);
        assert!(l.contains(&"item"));
        assert!(!l.contains(&"file"));
    }

    #[test]
    fn list_bracket_inside_item_content_no_completions() {
        // [[ inside list item content → no completions; media/links need [[#
        let c = completions("{{{#list\n  [[ [[");
        assert!(
            c.is_empty(),
            "bare [[ inside item content should produce no completions: {c:?}"
        );
    }

    #[test]
    fn list_bracket_hash_item_level_empty() {
        let c = completions("{{{#list\n  [[#");
        assert!(c.is_empty());
    }

    #[test]
    fn list_bracket_hash_inside_item_is_generic() {
        let c = completions("{{{#list\n  [[ [[#");
        let l = labels(&c);
        assert!(l.contains(&"youtube"));
    }

    // ── Fold context ──────────────────────────────────────────────

    #[test]
    fn fold_bracket_first_section() {
        // [[  at fold depth 1 → section template (no "body"/"header" keyword)
        let c = completions("{{{#fold\n  [[");
        let l = labels(&c);
        assert!(l.contains(&"section"), "expected 'section' template: {l:?}");
        assert!(!l.contains(&"file"));
    }

    #[test]
    fn fold_bracket_second_section() {
        // [[header]] [[  → still depth 1 (first pair balanced) → section template
        let c = completions("{{{#fold\n  [[header]] [[");
        let l = labels(&c);
        assert!(l.contains(&"section"), "expected 'section' template: {l:?}");
        assert!(!l.contains(&"file"));
    }

    #[test]
    fn fold_bracket_inside_section_no_completions() {
        // [[ [[  → depth 2 → inside fold section content → no completions (needs [[#)
        let c = completions("{{{#fold\n  [[ [[");
        assert!(
            c.is_empty(),
            "bare [[ inside fold section should produce no completions: {c:?}"
        );
    }

    // ── Nesting: list in table cell ───────────────────────────────

    #[test]
    fn list_in_table_cell_bracket_shows_item() {
        let prefix = "{{{#table\n  [[ [[\n    {{{#list\n      [[";
        let c = completions(prefix);
        let l = labels(&c);
        assert!(l.contains(&"item"), "expected list item completion: {l:?}");
        assert!(
            !l.contains(&"row"),
            "should not show row inside list: {l:?}"
        );
    }

    #[test]
    fn list_in_table_brace_hash_structural_only_if() {
        let prefix = "{{{#table\n  [[ [[\n    {{{#list\n      {{{#";
        let c = completions(prefix);
        let l = labels(&c);
        assert!(l.contains(&"if"));
        assert!(!l.contains(&"code"));
    }

    // ── Existing parameter completions still work ─────────────────

    #[test]
    fn youtube_param_completions() {
        let c = completions("[[#youtube #");
        let l = labels(&c);
        assert!(l.contains(&"id"));
        assert!(l.contains(&"autoplay"));
    }

    #[test]
    fn spotify_has_no_id() {
        let c = completions("[[#spotify #");
        let l = labels(&c);
        assert!(l.contains(&"track"));
        assert!(!l.contains(&"id"));
    }

    #[test]
    fn brace_code_param() {
        let c = completions("{{{#code #");
        assert!(c.iter().any(|c| c.label == "lang"));
    }

    #[test]
    fn flag_param_no_equals() {
        let c = completions("[[#youtube #");
        let autoplay = c.iter().find(|c| c.label == "autoplay").unwrap();
        assert_eq!(autoplay.insert_text.as_deref(), Some("autoplay"));
        let id = c.iter().find(|c| c.label == "id").unwrap();
        assert_eq!(id.insert_text.as_deref(), Some("id=\"$1\""));
    }

    #[test]
    fn closed_bracket_no_param_completions() {
        let c = completions("[[#youtube #id=\"abc\"]] #");
        assert!(c.is_empty());
    }

    #[test]
    fn macro_completions_after_single_bracket() {
        let c = completions("hello [");
        let l = labels(&c);
        assert!(l.contains(&"var"));
        assert!(l.contains(&"br"));
    }

    #[test]
    fn no_completions_for_plain_text() {
        assert!(completions("hello world").is_empty());
    }
}
