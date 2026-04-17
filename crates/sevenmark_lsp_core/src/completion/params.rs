use std::collections::BTreeSet;

use ls_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

use super::context::{
    CompletionContext, detect_brace_element, detect_bracket_element, in_unclosed_bracket,
    in_unclosed_styled_brace,
};

/// `(name, description, is_flag)` - flags insert just the name, values insert `name="$1"`.
type ParamDef = (&'static str, &'static str, bool);

const STYLE_PARAM_DEFS: &[ParamDef] = &[
    ("style", "Raw CSS declarations", false),
    ("color", "Text color", false),
    ("bgcolor", "Background color", false),
    ("size", "Font size", false),
    ("opacity", "Opacity", false),
    ("width", "CSS width", false),
    ("height", "CSS height", false),
    ("dark-style", "Dark-mode CSS declarations", false),
    ("dark-color", "Dark-mode text color", false),
    ("dark-bgcolor", "Dark-mode background color", false),
    ("dark-size", "Dark-mode font size", false),
    ("dark-opacity", "Dark-mode opacity", false),
    ("dark-width", "Dark-mode CSS width", false),
    ("dark-height", "Dark-mode CSS height", false),
];

const CLASS_PARAM_DEFS: &[ParamDef] = &[("class", "Extra CSS classes", false)];
const TABLE_CELL_PARAM_DEFS: &[ParamDef] = &[("x", "Column span", false), ("y", "Row span", false)];
const TABLE_ROW_PARAM_DEFS: &[ParamDef] = &[("head", "Header row (renders as <thead>/<th>)", true)];

const TABLE_PARAM_DEFS: &[ParamDef] = &[
    ("caption", "Table caption", false),
    (
        "wrapper-align",
        "Table wrapper alignment (left/center/right)",
        false,
    ),
    ("wrapper-width", "Table wrapper width", false),
    ("wrapper-style", "CSS style for the table wrapper", false),
    (
        "wrapper-dark-style",
        "Dark-mode CSS style for the table wrapper",
        false,
    ),
    ("sortable", "Enable column sorting", true),
];

const LIST_KIND_PARAM_DEFS: &[ParamDef] = &[
    ("1", "Ordered list: 1, 2, 3", true),
    ("a", "Ordered list: a, b, c", true),
    ("A", "Ordered list: A, B, C", true),
    ("i", "Ordered list: i, ii, iii", true),
    ("I", "Ordered list: I, II, III", true),
];

const CODE_PARAM_DEFS: &[ParamDef] = &[("lang", "Programming language", false)];
const TEX_PARAM_DEFS: &[ParamDef] = &[("block", "Display-style TeX block", true)];
const RUBY_PARAM_DEFS: &[ParamDef] = &[("ruby", "Ruby text annotation", false)];
const FOOTNOTE_PARAM_DEFS: &[ParamDef] = &[
    ("display", "Custom marker text", false),
    ("name", "Reusable named footnote identifier", false),
];
const NAMESPACE_PARAM_DEFS: &[ParamDef] = &[("namespace", "Target namespace", false)];

const MEDIA_TARGET_PARAM_DEFS: &[ParamDef] = &[
    ("file", "File path", false),
    ("document", "Document path", false),
    ("category", "Category name", false),
    ("user", "User name", false),
    ("url", "External URL", false),
];

const MEDIA_EXTRA_PARAM_DEFS: &[ParamDef] = &[
    ("anchor", "Anchor fragment", false),
    ("theme", "Theme visibility (light/dark)", false),
];

const YOUTUBE_PARAM_DEFS: &[ParamDef] = &[
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
];

const VIMEO_PARAM_DEFS: &[ParamDef] = &[
    ("id", "Video ID", false),
    ("h", "Privacy hash", false),
    ("width", "Player width", false),
    ("height", "Player height", false),
    ("autoplay", "Auto-play", true),
    ("loop", "Loop playback", true),
    ("mute", "Muted", true),
    ("color", "Player accent color", false),
    ("dnt", "Do-not-track", true),
];

const NICOVIDEO_PARAM_DEFS: &[ParamDef] = &[
    ("id", "Video ID", false),
    ("width", "Player width", false),
    ("height", "Player height", false),
    ("from", "Start time (seconds)", false),
    ("autoplay", "Auto-play", true),
];

const SPOTIFY_PARAM_DEFS: &[ParamDef] = &[
    ("track", "Track ID", false),
    ("album", "Album ID", false),
    ("playlist", "Playlist ID", false),
    ("artist", "Artist ID", false),
    ("episode", "Episode ID", false),
    ("show", "Show / podcast ID", false),
    ("width", "Player width", false),
    ("height", "Player height", false),
    ("dark", "Dark theme", true),
    ("compact", "Compact layout", true),
];

const DISCORD_PARAM_DEFS: &[ParamDef] = &[
    ("id", "Widget / server ID", false),
    ("width", "Widget width", false),
    ("height", "Widget height", false),
    ("dark", "Dark theme", true),
];

pub(super) fn parameter_completions(
    prefix: &str,
    ctx: CompletionContext<'_>,
) -> Option<Vec<CompletionItem>> {
    if in_unclosed_bracket(prefix) {
        match ctx {
            Some(("table", 1)) => return Some(table_row_param_completions()),
            Some(("table", 2)) => return Some(table_cell_param_completions()),
            Some(("list", 1)) => return Some(list_item_param_completions()),
            Some(("fold", 1)) => return Some(fold_inner_param_completions()),
            _ => {}
        }
    }

    if let Some(kw) = detect_bracket_element(prefix) {
        return Some(bracket_param_completions(kw));
    }

    if in_unclosed_bracket(prefix) {
        return Some(generic_media_param_completions());
    }

    if let Some(kw) = detect_brace_element(prefix) {
        return Some(brace_param_completions(kw));
    }

    if in_unclosed_styled_brace(prefix) {
        return Some(styled_param_completions());
    }

    None
}

pub(super) fn table_cell_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[TABLE_CELL_PARAM_DEFS, STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
}

pub(super) fn table_row_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[TABLE_ROW_PARAM_DEFS, STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
}

pub(super) fn list_item_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
}

pub(super) fn fold_inner_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
}

pub(super) fn styled_brace_hash_completions() -> Vec<CompletionItem> {
    styled_param_completions()
        .into_iter()
        .map(|mut item| {
            if let Some(insert_text) = item.insert_text.take() {
                let mut snippet = insert_text;
                snippet.push_str(" $0}}}");
                item.insert_text = Some(snippet);
            }
            item.detail = item.detail.map(|detail| format!("Styled block {detail}"));
            item
        })
        .collect()
}

fn styled_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
}

fn generic_media_param_completions() -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[
        MEDIA_TARGET_PARAM_DEFS,
        MEDIA_EXTRA_PARAM_DEFS,
        STYLE_PARAM_DEFS,
        CLASS_PARAM_DEFS,
    ])
}

fn bracket_param_completions(element: &str) -> Vec<CompletionItem> {
    match element {
        "youtube" => make_param_completions_from_groups(&[YOUTUBE_PARAM_DEFS, STYLE_PARAM_DEFS]),
        "vimeo" => make_param_completions_from_groups(&[VIMEO_PARAM_DEFS, STYLE_PARAM_DEFS]),
        "nicovideo" => {
            make_param_completions_from_groups(&[NICOVIDEO_PARAM_DEFS, STYLE_PARAM_DEFS])
        }
        "spotify" => make_param_completions_from_groups(&[SPOTIFY_PARAM_DEFS, STYLE_PARAM_DEFS]),
        "discord" => make_param_completions_from_groups(&[DISCORD_PARAM_DEFS, STYLE_PARAM_DEFS]),
        _ => generic_media_param_completions(),
    }
}

fn brace_param_completions(element: &str) -> Vec<CompletionItem> {
    match element {
        "code" => make_param_completions_from_groups(&[
            CODE_PARAM_DEFS,
            STYLE_PARAM_DEFS,
            CLASS_PARAM_DEFS,
        ]),
        "tex" => make_param_completions(TEX_PARAM_DEFS),
        "css" | "category" | "define" | "if" => Vec::new(),
        "table" => make_param_completions_from_groups(&[
            TABLE_PARAM_DEFS,
            STYLE_PARAM_DEFS,
            CLASS_PARAM_DEFS,
        ]),
        "list" => make_param_completions_from_groups(&[
            LIST_KIND_PARAM_DEFS,
            STYLE_PARAM_DEFS,
            CLASS_PARAM_DEFS,
        ]),
        "fold" | "quote" => {
            make_param_completions_from_groups(&[STYLE_PARAM_DEFS, CLASS_PARAM_DEFS])
        }
        "ruby" => make_param_completions_from_groups(&[
            RUBY_PARAM_DEFS,
            STYLE_PARAM_DEFS,
            CLASS_PARAM_DEFS,
        ]),
        "fn" => make_param_completions_from_groups(&[
            FOOTNOTE_PARAM_DEFS,
            STYLE_PARAM_DEFS,
            CLASS_PARAM_DEFS,
        ]),
        "include" | "redirect" => make_param_completions(NAMESPACE_PARAM_DEFS),
        // Unknown `{{{#name ...}}}` forms fall through to the styled element
        // parser as a flag parameter, so styled params are the useful follow-up.
        _ => styled_param_completions(),
    }
}

fn make_param_completions(params: &[ParamDef]) -> Vec<CompletionItem> {
    make_param_completions_from_groups(&[params])
}

fn make_param_completions_from_groups(groups: &[&[ParamDef]]) -> Vec<CompletionItem> {
    let mut seen = BTreeSet::new();
    let mut items = Vec::new();

    for group in groups {
        for &(name, detail, is_flag) in *group {
            if !seen.insert(name) {
                continue;
            }
            let snippet = if is_flag {
                name.to_string()
            } else {
                format!("{name}=\"$1\"")
            };
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(detail.to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text: Some(snippet),
                ..Default::default()
            });
        }
    }

    items
}
