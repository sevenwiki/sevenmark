use scraper::{Html, Selector};
use sevenmark_html::{RenderConfig, render_document};
use sevenmark_parser::core::parse_document;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tc/html")
}

fn render_config() -> RenderConfig<'static> {
    RenderConfig {
        file_base_url: Some("https://cdn.example.com/"),
        document_base_url: Some("/Document/"),
        category_base_url: Some("/Category/"),
        user_base_url: Some("/User/"),
    }
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn render_case(name: &str) -> String {
    let input_path = fixture_root().join("input").join(format!("{name}.sm"));
    let input = fs::read_to_string(&input_path).expect("fixture input should exist");
    let ast = parse_document(&normalize_newlines(&input));
    render_document(&ast, &render_config())
}

fn expected_case(name: &str) -> String {
    let expected_path = fixture_root().join("expected").join(format!("{name}.html"));
    normalize_newlines(
        &fs::read_to_string(&expected_path).expect("fixture expected html should exist"),
    )
}

fn assert_snapshot(name: &str) -> String {
    let actual = render_case(name);
    let expected = expected_case(name);
    assert_eq!(
        normalize_newlines(&actual).trim(),
        expected.trim(),
        "HTML fixture '{name}' did not match expected output"
    );
    actual
}

fn selector(value: &str) -> Selector {
    Selector::parse(value).expect("valid selector")
}

#[test]
fn renders_section_outline_fixture() {
    let html = assert_snapshot("section_outline");
    let doc = Html::parse_fragment(&html);

    let details = doc
        .select(&selector("details.sm-section"))
        .collect::<Vec<_>>();
    assert_eq!(details.len(), 3, "expected three nested section containers");
    assert_eq!(
        details
            .iter()
            .filter(|node| node.value().attr("open").is_some())
            .count(),
        2,
        "expected only authored open sections to carry the open attribute"
    );

    let folded = doc
        .select(&selector("details.sm-folded"))
        .collect::<Vec<_>>();
    assert_eq!(folded.len(), 1, "expected one folded section");
    assert!(
        folded[0].value().attr("open").is_none(),
        "folded sections should omit the open attribute"
    );
}

#[test]
fn renders_table_caption_fixture() {
    let html = assert_snapshot("table_caption");
    let doc = Html::parse_fragment(&html);

    let tables = doc.select(&selector("table.sm-table")).collect::<Vec<_>>();
    assert_eq!(tables.len(), 1, "expected a single rendered table");
    assert_eq!(
        tables[0].value().attr("data-sortable"),
        Some("true"),
        "sortable tables should carry the data-sortable flag"
    );

    let caption_text = doc
        .select(&selector("table.sm-table > caption"))
        .next()
        .map(|node| node.text().collect::<String>());
    assert_eq!(caption_text.as_deref(), Some("Scores"));

    assert_eq!(
        doc.select(&selector("table.sm-table thead th")).count(),
        2,
        "expected two header cells in the table head"
    );
    assert_eq!(
        doc.select(&selector("table.sm-table tbody tr")).count(),
        2,
        "expected two table body rows"
    );
}

#[test]
fn renders_sanitized_css_fixture() {
    let html = assert_snapshot("sanitized_css");
    let doc = Html::parse_fragment(&html);

    let styles = doc.select(&selector("style.sm-css")).collect::<Vec<_>>();
    assert_eq!(styles.len(), 1, "expected a single style element");
    assert!(
        html.contains("<\\/style>"),
        "embedded closing style tags should stay escaped"
    );
    assert!(
        !html.contains("url("),
        "unsafe url() values should be removed"
    );
    assert!(
        !html.contains("body{"),
        "bare tag selectors should be stripped"
    );
}

#[test]
fn renders_named_footnotes_fixture() {
    let html = assert_snapshot("named_footnotes");
    let doc = Html::parse_fragment(&html);

    assert_eq!(
        doc.select(&selector("sup.sm-footnote")).count(),
        3,
        "expected two named refs and one unnamed ref"
    );
    assert_eq!(
        doc.select(&selector("section.sm-footnotes li")).count(),
        2,
        "duplicate named references should not duplicate footnote entries"
    );

    let ids: HashSet<_> = doc
        .select(&selector("sup.sm-footnote"))
        .filter_map(|node| node.value().attr("id"))
        .collect();
    assert_eq!(ids.len(), 3, "footnote reference ids should remain unique");
}
