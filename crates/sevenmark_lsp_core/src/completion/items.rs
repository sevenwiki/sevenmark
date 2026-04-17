use ls_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

use super::context::CompletionContext;
use super::params::{
    fold_inner_param_completions, list_item_param_completions, styled_brace_hash_completions,
    table_cell_param_completions, table_row_param_completions,
};

/// `{{{#` trigger - which brace elements are valid here?
pub(super) fn brace_hash_completions(ctx: CompletionContext<'_>) -> Vec<CompletionItem> {
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
        let mut items = brace_keyword_completions();
        items.extend(styled_brace_hash_completions());
        items
    }
}

/// `[[` trigger - structural contexts only.
///
/// At content levels, bare `[[text]]` (no `#` params) renders as empty HTML,
/// so no completions are offered. Only structural positions within `{{{#table}}`,
/// `{{{#list}}}` and `{{{#fold}}}` get context-aware snippets; everything else
/// returns nothing (use `[[#` for actual links/media).
pub(super) fn bracket_completions_ctx(
    ctx: CompletionContext<'_>,
    _pos: Position,
) -> Vec<CompletionItem> {
    match ctx {
        Some(("table", 1)) => table_row_completions(),
        Some(("table", 2)) => table_cell_completions(),
        Some(("list", 1)) => list_item_completions(),
        Some(("fold", 1)) => fold_section_completions(),
        _ => Vec::new(),
    }
}

/// `[[#` trigger - keyword after `#` or element-specific params.
pub(super) fn bracket_hash_completions_ctx(
    ctx: CompletionContext<'_>,
    pos: Position,
) -> Vec<CompletionItem> {
    match ctx {
        Some(("table", 1)) => table_row_param_completions(),
        Some(("table", 2)) => table_cell_param_completions(),
        Some(("table", _)) => generic_bracket_hash_completions(pos),
        Some(("list", 1)) => list_item_param_completions(),
        Some(("list", _)) => generic_bracket_hash_completions(pos),
        Some(("fold", 1)) => fold_inner_param_completions(),
        Some(("fold", _)) => generic_bracket_hash_completions(pos),
        _ => generic_bracket_hash_completions(pos),
    }
}

pub(super) fn macro_completions(_pos: Position) -> Vec<CompletionItem> {
    let macros = [
        ("var", "var($1)]", "Variable reference"),
        ("br", "br]", "Line break"),
        ("clear", "clear]", "Float clear"),
        ("null", "null]", "Null (no output)"),
        ("fn", "fn]", "Render collected footnotes"),
        ("now", "now]", "Current time"),
        ("age", "age($1)]", "Age calculation"),
        ("anchor", "anchor($1)]", "Named anchor"),
        ("date", "date]", "Current date"),
        ("datetime", "datetime]", "Current date and time"),
        ("dday", "dday($1)]", "D-day counter"),
        ("pagecount", "pagecount]", "Total page count"),
        ("toc", "toc]", "Table of contents"),
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

fn brace_keyword_completions() -> Vec<CompletionItem> {
    let keywords = [
        ("code", "code #lang=\"$1\"\n$0\n}}}", "Code block"),
        ("tex", "tex\n$0\n}}}", "TeX block"),
        ("css", "css\n$0\n}}}", "CSS block"),
        ("table", "table\n$0\n}}}", "Table"),
        ("list", "list\n$0\n}}}", "List"),
        ("fold", "fold\n$0\n}}}", "Fold (collapsible)"),
        ("quote", "quote\n$0\n}}}", "Block quote"),
        ("define", "define #$1=\"$2\"}}}", "Variable definition"),
        ("if", "if $1 ::\n$0\n}}}", "Conditional block"),
        ("include", "include $0}}}", "Document inclusion"),
        ("category", "category $0}}}", "Category"),
        ("redirect", "redirect $0}}}", "Redirect"),
        ("ruby", "ruby #ruby=\"$1\" $0}}}", "Ruby annotation"),
        ("fn", "fn $0}}}", "Footnote"),
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

/// Only `{{{#if` - valid at structural levels of table / list.
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

/// `[[#` at top-level / content - keyword already typed.
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
            "Table cell with span",
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

/// `[[` at fold section level - just provide a closing template.
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
