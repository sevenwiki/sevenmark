use sevenmark::parse_document;
use std::fs;
fn parse_file_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = parse_document(content);
    let json = serde_json::to_string_pretty(&result)?;
    Ok(json)
}

fn run_fold_test(test_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = format!("tests/fold/input/{}.txt", test_name);
    let expected_path = format!("tests/fold/expected/{}.json", test_name);

    // 입력 파일 읽기
    let input_content = fs::read_to_string(&input_path)?;

    // 파싱 실행
    let actual_output = parse_file_content(&input_content)?;

    // 예상 결과 파일이 있어야 함
    let expected_output = fs::read_to_string(&expected_path)
        .map_err(|_| format!("Expected output file not found: {}", expected_path))?;

    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Test '{}' failed: output doesn't match expected",
        test_name
    );

    Ok(())
}

#[test]
fn test_basic_fold() {
    run_fold_test("basic_fold").expect("basic_fold test failed");
}

#[test]
fn test_fold_with_params() {
    run_fold_test("fold_with_params").expect("fold_with_params test failed");
}

#[test]
fn test_fold_with_formatting() {
    run_fold_test("fold_with_formatting").expect("fold_with_formatting test failed");
}
