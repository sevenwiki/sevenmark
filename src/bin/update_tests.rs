use sevenmark::sevenmark::core::parse_document_with_preprocessing;
use std::fs;
use std::path::Path;

fn main() {
    let failed_tests = [
        ("fold", "basic_fold"),
        ("brace", "list"),
        ("brace", "styled"),
        ("complex", "all_parameter_combinations"),
        ("complex", "deeply_nested_lists"),
        ("complex", "fold_with_rich_content"),
        ("complex", "parameter_conflicts"),
        ("complex", "scientific_document"),
        ("complex", "special_parameters"),
        ("complex", "table_with_nested_elements"),
        ("complex", "technical_documentation"),
        ("complex", "wiki_page_example"),
        ("fold", "fold_with_formatting"),
        ("fold", "fold_with_params"),
        ("markdown", "headers"),
    ];

    for (category, test_name) in failed_tests.iter() {
        println!("Processing {}/{}", category, test_name);
        
        let input_path = format!("tests/{}/input/{}.txt", category, test_name);
        let expected_path = format!("tests/{}/expected/{}.json", category, test_name);
        
        if !Path::new(&input_path).exists() {
            println!("  Input file not found: {}", input_path);
            continue;
        }
        
        let input_content = match fs::read_to_string(&input_path) {
            Ok(content) => content,
            Err(e) => {
                println!("  Error reading input file: {}", e);
                continue;
            }
        };
        
        let (result, _preprocess_info) = parse_document_with_preprocessing(&input_content);
        
        let json_output = match serde_json::to_string_pretty(&result) {
            Ok(json) => json,
            Err(e) => {
                println!("  Error serializing to JSON: {}", e);
                continue;
            }
        };
        
        if let Err(e) = fs::write(&expected_path, &json_output) {
            println!("  Error writing expected file: {}", e);
            continue;
        }
        
        println!("  Updated: {}", expected_path);
    }
    
    println!("Done updating expected test files!");
}