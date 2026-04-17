use ls_types::{CompletionItem, Position};

use super::context::context_and_bracket_depth;
use super::get_completions;
use crate::document::DocumentState;

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

#[test]
fn context_top_level_is_none() {
    assert!(context_and_bracket_depth("hello world").is_none());
}

#[test]
fn context_multibyte_chars_do_not_panic() {
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
    assert_eq!(
        context_and_bracket_depth("{{{#table\n  [[ [[c]] ]]\n  [["),
        Some(("table", 1))
    );
}

#[test]
fn context_list_in_table_cell() {
    let prefix = "{{{#table\n  [[ [[\n    {{{#list\n      [[";
    assert_eq!(context_and_bracket_depth(prefix), Some(("list", 1)));
}

#[test]
fn context_table_in_list_item() {
    let prefix = "{{{#list\n  [[\n    {{{#table\n      [[";
    assert_eq!(context_and_bracket_depth(prefix), Some(("table", 1)));
}

#[test]
fn context_after_closed_inner_returns_outer() {
    let prefix = "{{{#table\n  [[ [[\n    {{{#list [[item]] }}}\n    [[";
    assert_eq!(context_and_bracket_depth(prefix), Some(("table", 3)));
}

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
    assert!(l.contains(&"quote"));
    assert!(!l.contains(&"blockquote"));
    assert!(l.contains(&"fn"));
    assert!(!l.contains(&"footnote"));
    assert!(l.contains(&"style"));
    assert!(l.contains(&"if"));
    assert!(!l.contains(&"literal"));
}

#[test]
fn styled_brace_hash_inserts_parameter_not_fake_keyword() {
    let c = completions("{{{#");
    let style = c.iter().find(|c| c.label == "style").unwrap();
    assert_eq!(style.insert_text.as_deref(), Some("style=\"$1\" $0}}}"));
}

#[test]
fn styled_spaced_hash_suggests_common_style_params() {
    let c = completions("{{{ #");
    let l = labels(&c);
    assert!(l.contains(&"style"));
    assert!(l.contains(&"color"));
    assert!(l.contains(&"bgcolor"));
    assert!(l.contains(&"class"));
}

#[test]
fn top_level_bracket_no_completions() {
    let c = completions("hello [[");
    assert!(c.is_empty(), "bare [[ should produce no completions: {c:?}");
}

#[test]
fn top_level_bracket_hash_suggests_generic_no_link() {
    let c = completions("hello [[#");
    let l = labels(&c);
    assert!(l.contains(&"file"));
    assert!(l.contains(&"youtube"));
    assert!(!l.contains(&"link"));
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

#[test]
fn table_brace_hash_structural_suggests_only_if() {
    let c = completions("{{{#table\n  {{{#");
    let l = labels(&c);
    assert!(l.contains(&"if"));
    assert!(!l.contains(&"code"));
    assert!(!l.contains(&"table"));
}

#[test]
fn table_brace_hash_inside_row_suggests_only_if() {
    let c = completions("{{{#table\n  [[ {{{#");
    let l = labels(&c);
    assert!(l.contains(&"if"));
    assert!(!l.contains(&"code"));
}

#[test]
fn table_brace_hash_inside_cell_content_suggests_all() {
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
    let c = completions("{{{#table\n  [[ [[ [[");
    assert!(
        c.is_empty(),
        "bare [[ inside cell content should produce no completions: {c:?}"
    );
}

#[test]
fn table_bracket_hash_row_level_shows_head() {
    let c = completions("{{{#table\n  [[#");
    let l = labels(&c);
    assert!(l.contains(&"head"), "row level [[# should show head flag");
}

#[test]
fn table_bracket_hash_cell_level_shows_xy() {
    let c = completions("{{{#table\n  [[ [[#");
    let l = labels(&c);
    assert!(l.contains(&"x"));
    assert!(l.contains(&"y"));
    assert!(l.contains(&"style"));
    assert!(l.contains(&"dark-style"));
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
    assert!(l.contains(&"style"));
    assert!(l.contains(&"dark-style"));
}

#[test]
fn table_row_spaced_hash_shows_head() {
    let c = completions("{{{#table\n  [[ #");
    let l = labels(&c);
    assert!(l.contains(&"head"));
    assert!(l.contains(&"style"));
    assert!(!l.contains(&"x"));
}

#[test]
fn table_brace_params_include_wrapper_params() {
    let c = completions("{{{#table #");
    let l = labels(&c);
    assert!(l.contains(&"wrapper-align"));
    assert!(l.contains(&"wrapper-width"));
    assert!(l.contains(&"wrapper-style"));
    assert!(l.contains(&"wrapper-dark-style"));
    assert!(l.contains(&"caption"));
    assert!(l.contains(&"sortable"));
}

#[test]
fn table_outside_closed_bracket_no_completions() {
    let c = completions("{{{#table\n  [[ [[c]] ]]\n}}}\n\n[[");
    assert!(
        c.is_empty(),
        "bare [[ after closed table should produce no completions: {c:?}"
    );
}

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
    let c = completions("{{{#list\n  [[ [[");
    assert!(
        c.is_empty(),
        "bare [[ inside item content should produce no completions: {c:?}"
    );
}

#[test]
fn list_bracket_hash_item_level_shows_item_params() {
    let c = completions("{{{#list\n  [[#");
    let l = labels(&c);
    assert!(l.contains(&"style"));
    assert!(l.contains(&"class"));
    assert!(!l.contains(&"youtube"));
}

#[test]
fn list_bracket_hash_inside_item_is_generic() {
    let c = completions("{{{#list\n  [[ [[#");
    let l = labels(&c);
    assert!(l.contains(&"youtube"));
}

#[test]
fn fold_bracket_first_section() {
    let c = completions("{{{#fold\n  [[");
    let l = labels(&c);
    assert!(l.contains(&"section"), "expected 'section' template: {l:?}");
    assert!(!l.contains(&"file"));
}

#[test]
fn fold_bracket_second_section() {
    let c = completions("{{{#fold\n  [[header]] [[");
    let l = labels(&c);
    assert!(l.contains(&"section"), "expected 'section' template: {l:?}");
    assert!(!l.contains(&"file"));
}

#[test]
fn fold_bracket_inside_section_no_completions() {
    let c = completions("{{{#fold\n  [[ [[");
    assert!(
        c.is_empty(),
        "bare [[ inside fold section should produce no completions: {c:?}"
    );
}

#[test]
fn fold_bracket_hash_section_shows_inner_params() {
    let c = completions("{{{#fold\n  [[#");
    let l = labels(&c);
    assert!(l.contains(&"style"));
    assert!(l.contains(&"class"));
    assert!(!l.contains(&"youtube"));
}

#[test]
fn fold_bracket_hash_inside_section_is_generic() {
    let c = completions("{{{#fold\n  [[ [[#");
    let l = labels(&c);
    assert!(l.contains(&"youtube"));
    assert!(l.contains(&"file"));
    assert!(!l.contains(&"style"));
}

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
    assert!(c.iter().any(|c| c.label == "style"));
}

#[test]
fn tex_param_shows_block_flag() {
    let c = completions("{{{#tex #");
    let l = labels(&c);
    assert!(l.contains(&"block"));
}

#[test]
fn list_block_params_include_order_flags() {
    let c = completions("{{{#list #");
    let l = labels(&c);
    assert!(l.contains(&"1"));
    assert!(l.contains(&"a"));
    assert!(l.contains(&"I"));
    assert!(l.contains(&"style"));
}

#[test]
fn footnote_params_use_fn_keyword() {
    let c = completions("{{{#fn #");
    let l = labels(&c);
    assert!(l.contains(&"display"));
    assert!(l.contains(&"name"));
    assert!(l.contains(&"style"));
}

#[test]
fn include_and_redirect_show_namespace_param() {
    let include = completions("{{{#include #");
    let redirect = completions("{{{#redirect #");
    assert!(labels(&include).contains(&"namespace"));
    assert!(labels(&redirect).contains(&"namespace"));
    assert!(!labels(&include).contains(&"style"));
    assert!(!labels(&redirect).contains(&"style"));
}

#[test]
fn generic_media_spaced_hash_suggests_media_params() {
    let c = completions("[[ #");
    let l = labels(&c);
    assert!(l.contains(&"file"));
    assert!(l.contains(&"document"));
    assert!(l.contains(&"url"));
    assert!(l.contains(&"anchor"));
    assert!(l.contains(&"theme"));
}

#[test]
fn brace_css_has_no_parameter_completions() {
    let c = completions("{{{#css #");
    assert!(
        c.is_empty(),
        "css blocks should not offer parameter completions: {c:?}"
    );
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
    assert!(l.contains(&"clear"));
    assert!(l.contains(&"toc"));
}

#[test]
fn no_completions_for_plain_text() {
    assert!(completions("hello world").is_empty());
}
