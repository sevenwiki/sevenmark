use sevenmark::parse_document;
use std::fs;

fn parse_file_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = parse_document(content);
    let json = serde_json::to_string_pretty(&result)?;
    Ok(json)
}

fn run_parser_test(category: &str, test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = format!("tests/{}/input/{}.txt", category, test_name);
    let expected_path = format!("tests/{}/expected/{}.json", category, test_name);

    // 입력 파일 읽기
    let input_content = fs::read_to_string(&input_path)?;

    // 파싱 실행
    let actual_output = parse_file_content(&input_content)?;

    // 예상 결과와 비교
    let expected_output = fs::read_to_string(&expected_path)
        .map_err(|_| format!("Expected output file not found: {}", expected_path))?;

    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Test '{}/{}' failed: output doesn't match expected",
        category,
        test_name
    );

    Ok(())
}

// Fold Tests
#[test]
fn test_basic_fold() {
    run_parser_test("fold", "basic_fold").expect("basic fold test failed");
}

#[test]
fn test_fold_with_params() {
    run_parser_test("fold", "fold_with_params").expect("fold with params test failed");
}

#[test]
fn test_fold_with_formatting() {
    run_parser_test("fold", "fold_with_formatting").expect("fold with formatting test failed");
}

// Brace Tests
#[test]
fn test_brace_literal() {
    run_parser_test("brace", "literal").expect("brace literal test failed");
}

#[test]
fn test_brace_styled() {
    run_parser_test("brace", "styled").expect("brace styled test failed");
}

#[test]
fn test_brace_table() {
    run_parser_test("brace", "table").expect("brace table test failed");
}

#[test]
fn test_brace_list() {
    run_parser_test("brace", "list").expect("brace list test failed");
}

#[test]
fn test_brace_blockquote() {
    run_parser_test("brace", "blockquote").expect("brace blockquote test failed");
}

#[test]
fn test_brace_code() {
    run_parser_test("brace", "code").expect("brace code test failed");
}

#[test]
fn test_brace_tex() {
    run_parser_test("brace", "tex").expect("brace tex test failed");
}

#[test]
fn test_brace_include() {
    run_parser_test("brace", "include").expect("brace include test failed");
}

#[test]
fn test_brace_category() {
    run_parser_test("brace", "category").expect("brace category test failed");
}

// Markdown Tests
#[test]
fn test_markdown_formatting() {
    run_parser_test("markdown", "formatting").expect("markdown formatting test failed");
}

#[test]
fn test_markdown_headers() {
    run_parser_test("markdown", "headers").expect("markdown headers test failed");
}

#[test]
fn test_markdown_hline() {
    run_parser_test("markdown", "hline").expect("markdown hline test failed");
}

// Macro Tests
#[test]
fn test_macro_time_macros() {
    run_parser_test("macro", "time_macros").expect("macro time_macros test failed");
}

#[test]
fn test_macro_utility_macros() {
    run_parser_test("macro", "utility_macros").expect("macro utility_macros test failed");
}

// Comment Tests
#[test]
fn test_comment_inline_comment() {
    run_parser_test("comment", "inline_comment").expect("comment inline test failed");
}

#[test]
fn test_comment_multiline_comment() {
    run_parser_test("comment", "multiline_comment").expect("comment multiline test failed");
}

// Escape Tests
#[test]
fn test_escape_chars() {
    run_parser_test("escape", "escape_chars").expect("escape chars test failed");
}

// Complex Tests
#[test]
fn test_complex_fold_with_rich_content() {
    run_parser_test("complex", "fold_with_rich_content")
        .expect("complex fold with rich content test failed");
}

#[test]
fn test_complex_table_with_nested_elements() {
    run_parser_test("complex", "table_with_nested_elements")
        .expect("complex table with nested elements test failed");
}

#[test]
fn test_complex_deeply_nested_lists() {
    run_parser_test("complex", "deeply_nested_lists")
        .expect("complex deeply nested lists test failed");
}

#[test]
fn test_complex_all_parameter_combinations() {
    run_parser_test("complex", "all_parameter_combinations")
        .expect("complex all parameter combinations test failed");
}

#[test]
fn test_complex_parameter_conflicts() {
    run_parser_test("complex", "parameter_conflicts")
        .expect("complex parameter conflicts test failed");
}

#[test]
fn test_complex_special_parameters() {
    run_parser_test("complex", "special_parameters")
        .expect("complex special parameters test failed");
}

#[test]
fn test_complex_technical_documentation() {
    run_parser_test("complex", "technical_documentation")
        .expect("complex technical documentation test failed");
}

#[test]
fn test_complex_wiki_page_example() {
    run_parser_test("complex", "wiki_page_example").expect("complex wiki page example test failed");
}

#[test]
fn test_complex_scientific_document() {
    run_parser_test("complex", "scientific_document")
        .expect("complex scientific document test failed");
}
