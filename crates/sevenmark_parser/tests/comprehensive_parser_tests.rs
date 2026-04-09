use sevenmark_ast::Element;
use sevenmark_parser::core::parse_document;
use std::fs;
use std::path::Path;

fn parse_file_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = parse_document(content);
    let json = serde_json::to_string_pretty(&result)?;
    Ok(json)
}

fn run_parser_test(category: &str, test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_path = format!(
        "{}/../../tc/{}/input/{}.sm",
        manifest_dir, category, test_name
    );
    let expected_path = format!(
        "{}/../../tc/{}/expected/{}.json",
        manifest_dir, category, test_name
    );

    // 입력 파일 읽기 - Normalize CRLF to LF for consistent byte offsets
    let input_content = fs::read_to_string(&input_path)?.replace("\r\n", "\n");

    // 파싱 실행
    let actual_output = parse_file_content(&input_content)?;

    // 예상 결과와 비교
    let expected_output = fs::read_to_string(&expected_path)
        .map_err(|_| format!("Expected output file not found: {}", expected_path))?;

    // Normalize line endings (CRLF -> LF) for cross-platform compatibility
    let actual_normalized = actual_output.replace("\r\n", "\n");
    let expected_normalized = expected_output.replace("\r\n", "\n");

    assert_eq!(
        actual_normalized.trim(),
        expected_normalized.trim(),
        "Test '{}/{}' failed: output doesn't match expected",
        category,
        test_name
    );

    Ok(())
}

fn fixture_names_for_category(category: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tc")
        .join(category)
        .join("input");
    let mut fixture_names = Vec::new();

    for entry in fs::read_dir(&fixtures_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("sm") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("Invalid fixture filename: {}", path.display()))?;
        fixture_names.push(name.to_string());
    }

    fixture_names.sort_unstable();
    Ok(fixture_names)
}

fn run_parser_category(category: &str) -> Result<(), Box<dyn std::error::Error>> {
    let fixture_names = fixture_names_for_category(category)?;
    assert!(
        !fixture_names.is_empty(),
        "No parser fixtures found for category '{category}'"
    );

    for fixture_name in fixture_names {
        run_parser_test(category, &fixture_name)?;
    }

    Ok(())
}

#[test]
fn test_fold_fixtures() {
    run_parser_category("fold").expect("fold fixture tests failed");
}

#[test]
fn test_brace_fixtures() {
    run_parser_category("brace").expect("brace fixture tests failed");
}

#[test]
fn test_markdown_fixtures() {
    run_parser_category("markdown").expect("markdown fixture tests failed");
}

#[test]
fn test_macro_fixtures() {
    run_parser_category("macro").expect("macro fixture tests failed");
}

#[test]
fn test_comment_fixtures() {
    run_parser_category("comment").expect("comment fixture tests failed");
}

#[test]
fn test_escape_fixtures() {
    run_parser_category("escape").expect("escape fixture tests failed");
}

#[test]
fn test_if_fixtures() {
    run_parser_category("if").expect("if fixture tests failed");
}

#[test]
fn test_complex_fixtures() {
    run_parser_category("complex").expect("complex fixture tests failed");
}

#[test]
fn test_invalid_fixtures() {
    run_parser_category("invalid").expect("invalid fixture tests failed");
}

#[test]
fn test_raw_code_crlf_backslash_before_closer() {
    let input = "{{{#code\r\nfirst\r\n\\}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let code = parsed
        .iter()
        .find_map(|e| match e {
            Element::Code(c) => Some(c),
            _ => None,
        })
        .expect("expected Code element");

    assert_eq!(code.value, "first\r\n\\");
}

#[test]
fn test_raw_tex_crlf_backslash_before_closer() {
    let input = "{{{#tex #block\r\n\\}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let tex = parsed
        .iter()
        .find_map(|e| match e {
            Element::TeX(t) => Some(t),
            _ => None,
        })
        .expect("expected TeX element");

    assert_eq!(tex.value, "\\");
}

#[test]
fn test_raw_css_crlf_backslash_before_closer() {
    let input = "{{{#css\r\n.a { color: red; }\r\n\\}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let css = parsed
        .iter()
        .find_map(|e| match e {
            Element::Css(c) => Some(c),
            _ => None,
        })
        .expect("expected Css element");

    assert_eq!(css.value, ".a { color: red; }\r\n\\");
}

#[test]
fn test_toc_macro() {
    let input = "[toc]\n";
    let parsed = parse_document(input);

    let toc = parsed
        .first()
        .and_then(|e| match e {
            Element::Toc(toc) => Some(toc),
            _ => None,
        })
        .expect("expected Toc element");

    assert_eq!(toc.span.start, 0);
    assert_eq!(toc.span.end, 5);
    assert!(
        matches!(parsed.get(1), Some(Element::SoftBreak(_))),
        "expected trailing newline to become SoftBreak: {parsed:#?}"
    );
}

#[test]
fn test_raw_code_balanced_triple_brace_matching() {
    let input = "{{{#code\nwhat{{{}}}{{{}}}\n}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let code = parsed
        .iter()
        .find_map(|e| match e {
            Element::Code(c) => Some(c),
            _ => None,
        })
        .expect("expected Code element");

    assert_eq!(code.value, "what{{{}}}{{{}}}\n");
}

#[test]
fn test_raw_tex_balanced_triple_brace_matching() {
    let input = "{{{#tex\n\\text{a{{{b}}}c}\n}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let tex = parsed
        .iter()
        .find_map(|e| match e {
            Element::TeX(t) => Some(t),
            _ => None,
        })
        .expect("expected TeX element");

    assert_eq!(tex.value, "\\text{a{{{b}}}c}\n");
}

#[test]
fn test_raw_css_balanced_triple_brace_matching() {
    let input = "{{{#css\n.a::after{content:\"{{{x}}}\";}\n}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let css = parsed
        .iter()
        .find_map(|e| match e {
            Element::Css(c) => Some(c),
            _ => None,
        })
        .expect("expected Css element");

    assert_eq!(css.value, ".a::after{content:\"{{{x}}}\";}\n");
}

#[test]
fn test_raw_code_balanced_triple_brace_with_utf8_content() {
    let input = "{{{#code\n한글🙂{{{중첩}}}끝\n}}}";
    let parsed = parse_document(input);

    assert!(
        !parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "unexpected parse error: {parsed:#?}"
    );

    let code = parsed
        .iter()
        .find_map(|e| match e {
            Element::Code(c) => Some(c),
            _ => None,
        })
        .expect("expected Code element");

    assert_eq!(code.value, "한글🙂{{{중첩}}}끝\n");
}

#[test]
fn test_redirect_stops_document_and_marks_trailing_content_as_error() {
    let input = "{{{#redirect TargetPage}}}\n# This should be ignored";
    let parsed = parse_document(input);

    assert_eq!(
        parsed.len(),
        2,
        "redirect + trailing content should yield Redirect + Error"
    );
    assert!(
        matches!(parsed.first(), Some(Element::Redirect(_))),
        "expected Redirect element, got: {parsed:#?}"
    );
    assert!(
        matches!(parsed.get(1), Some(Element::Error(_))),
        "expected trailing Error element, got: {parsed:#?}"
    );
}

#[test]
fn test_malformed_redirect_produces_error() {
    let input = "{{{#redirect";
    let parsed = parse_document(input);

    assert!(
        !parsed.is_empty() && parsed.iter().any(|e| matches!(e, Element::Error(_))),
        "malformed redirect must produce Error element, got: {parsed:#?}"
    );
}
