use super::resolver::{normalized_plain_text, resolve_media_elements};
use super::*;
use sevenmark_ast::{MediaElement, Parameter, Parameters, Span, TextElement};

fn span() -> Span {
    Span::synthesized()
}

fn text(value: &str) -> Element {
    Element::Text(TextElement {
        span: span(),
        value: value.to_string(),
    })
}

fn add_param(parameters: &mut Parameters, key: &str, value: &str) {
    parameters.insert(
        key.to_string(),
        Parameter {
            span: span(),
            key: key.to_string(),
            value: vec![text(value)],
        },
    );
}

fn media(params: &[(&str, &str)]) -> Element {
    let mut parameters = Parameters::new();
    for (k, v) in params {
        add_param(&mut parameters, k, v);
    }
    Element::Media(MediaElement {
        span: span(),
        open_span: span(),
        close_span: span(),
        parameters,
        children: Vec::new(),
        resolved_info: None,
    })
}

#[test]
fn normalized_plain_text_trims_values() {
    assert_eq!(
        normalized_plain_text(&[text("  abc \n")]),
        Some("abc".to_string())
    );
    assert_eq!(normalized_plain_text(&[text(" \n\t ")]), None);
}

#[test]
fn resolve_media_elements_matches_trimmed_document_title() {
    let mut ast = vec![media(&[
        ("document", " Page \n"),
        ("url", " https://example.com \n"),
    ])];
    let mut resolved_map = HashMap::new();
    resolved_map.insert(
        (DocumentNamespace::Document, "Page".to_string()),
        (None, None, None, true),
    );

    resolve_media_elements(&mut ast, &resolved_map);

    let media_elem = match &ast[0] {
        Element::Media(m) => m,
        _ => panic!("expected media element"),
    };
    let resolved = media_elem
        .resolved_info
        .as_ref()
        .expect("resolved media info should exist");
    assert_eq!(
        resolved
            .document
            .as_ref()
            .map(|d| (d.title.as_str(), d.is_valid)),
        Some(("Page", true))
    );
    assert_eq!(resolved.url.as_deref(), Some("https://example.com"));
}

#[test]
fn resolve_media_elements_sets_file_dimensions_from_map() {
    let mut ast = vec![media(&[("file", " poster \n")])];
    let mut resolved_map = HashMap::new();
    resolved_map.insert(
        (DocumentNamespace::File, "poster".to_string()),
        (
            Some("files/poster.png".to_string()),
            Some(640),
            Some(360),
            true,
        ),
    );

    resolve_media_elements(&mut ast, &resolved_map);

    let media_elem = match &ast[0] {
        Element::Media(m) => m,
        _ => panic!("expected media element"),
    };
    let resolved = media_elem
        .resolved_info
        .as_ref()
        .expect("resolved media info should exist");
    assert_eq!(
        resolved
            .file
            .as_ref()
            .map(|f| (f.url.as_str(), f.width, f.height, f.is_valid)),
        Some(("files/poster.png", Some(640), Some(360), true))
    );
}
