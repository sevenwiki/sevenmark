use sevenmark::parse_document;
use sevenmark::sevenmark::visitor::monaco::convert_ast_to_monaco_json;
use std::fs;

fn monaco_parse_file_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = parse_document(content);
    let monaco_json = convert_ast_to_monaco_json(&result, content);
    
    Ok(monaco_json)
}

fn run_monaco_test(test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = format!("tests/monaco/input/{}.txt", test_name);
    let expected_path = format!("tests/monaco/expected/{}.json", test_name);

    let input_content = fs::read_to_string(&input_path)?;

    let actual_output = monaco_parse_file_content(&input_content)?;
    
    let expected_output = fs::read_to_string(&expected_path)
        .map_err(|_| format!("Expected Monaco output file not found: {}", expected_path))?;

    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Monaco test '{}' failed: output doesn't match expected",
        test_name
    );

    Ok(())
}

// Monaco Editor Tests - Line/Column Position Conversion
#[test]
fn test_monaco_basic_text() {
    run_monaco_test("basic_text").expect("Monaco basic text test failed");
}

#[test]
fn test_monaco_utf8_text() {
    run_monaco_test("utf8_text").expect("Monaco UTF-8 text test failed");
}

#[test]
fn test_monaco_markdown_elements() {
    run_monaco_test("markdown_elements").expect("Monaco markdown elements test failed");
}

#[test]
fn test_monaco_complex_elements() {
    run_monaco_test("complex_elements").expect("Monaco complex elements test failed");
}

#[test]
fn test_monaco_edge_cases() {
    run_monaco_test("edge_cases").expect("Monaco edge cases test failed");
}