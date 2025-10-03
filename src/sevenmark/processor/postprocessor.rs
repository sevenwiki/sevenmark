use crate::SevenMarkElement;
use crate::sevenmark::core::parse_document;
use crate::sevenmark::processor::wiki::WikiData;
use crate::sevenmark::{Location, TextElement, Traversable};
use std::collections::HashMap;

pub struct SevenMarkPostprocessor;

impl SevenMarkPostprocessor {
    /// Include 치환만 수행 (수집은 Preprocessor가 담당)
    pub fn apply(mut ast: Vec<SevenMarkElement>, wiki_data: &WikiData) -> Vec<SevenMarkElement> {
        Self::replace_includes(&mut ast, wiki_data);
        ast
    }

    /// Include 요소의 content를 파싱된 AST로 교체
    fn replace_includes(elements: &mut [SevenMarkElement], wiki_data: &WikiData) {
        for element in elements.iter_mut() {
            match element {
                SevenMarkElement::Include(include_elem) => {
                    let title = Self::extract_plain_text(&include_elem.content);
                    if !title.is_empty() {
                        // namespace 추출 및 key 생성
                        let namespace_str = include_elem
                            .parameters
                            .get("namespace")
                            .map(|param| Self::extract_plain_text(&param.value))
                            .filter(|s| !s.is_empty())
                            .unwrap_or_else(|| "Document".to_string());

                        let key = format!("{}:{}", namespace_str, title);
                        println!("  [Postprocessor] Processing include: {}", key);

                        // WikiData에서 Include된 문서 찾기
                        if let Some(include_data) = wiki_data.includes.get(&key) {
                            println!("    ✓ Found! Replacing content with parsed AST");

                            // 1. Include된 문서 파싱
                            let mut parsed_ast = parse_document(&include_data.content);

                            // 2. IncludeInfo에서 parameters 가져오기
                            let params = &include_data.info.parameters;

                            // 3. 파싱된 AST에서 Variable 요소를 parameter 값으로 치환
                            Self::substitute_variables(&mut parsed_ast, params);

                            // 4. Include 요소의 content를 치환된 AST로 교체
                            include_elem.content = parsed_ast;
                        } else {
                            println!("    ✗ NOT FOUND in WikiData");
                        }
                    }

                    // content 안의 Include도 처리 (중첩 Include)
                    Self::replace_includes(&mut include_elem.content, wiki_data);
                }
                _ => {
                    // 자식 요소들도 재귀 처리
                    Self::replace_includes_in_element(element, wiki_data);
                }
            }
        }
    }

    /// 단일 요소의 자식에서 Include 처리
    fn replace_includes_in_element(element: &mut SevenMarkElement, wiki_data: &WikiData) {
        element.traverse_children(&mut |child| {
            // 자식이 Include이면 직접 처리
            if let SevenMarkElement::Include(include_elem) = child {
                let title = Self::extract_plain_text(&include_elem.content);
                if !title.is_empty() {
                    let namespace_str = include_elem
                        .parameters
                        .get("namespace")
                        .map(|param| Self::extract_plain_text(&param.value))
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| "Document".to_string());

                    let key = format!("{}:{}", namespace_str, title);
                    println!("  [Postprocessor] Processing nested include: {}", key);

                    if let Some(include_data) = wiki_data.includes.get(&key) {
                        println!("    ✓ Found! Replacing content");
                        let mut parsed_ast = parse_document(&include_data.content);
                        let params = &include_data.info.parameters;
                        Self::substitute_variables(&mut parsed_ast, params);
                        include_elem.content = parsed_ast;
                    } else {
                        println!("    ✗ NOT FOUND in WikiData");
                    }
                }
            }
            // 재귀적으로 자식의 자식들도 처리
            Self::replace_includes_in_element(child, wiki_data);
        });
    }

    /// AST에서 Variable 요소를 parameter 값으로 치환
    fn substitute_variables(elements: &mut [SevenMarkElement], params: &HashMap<String, String>) {
        for element in elements.iter_mut() {
            Self::substitute_variable_in_element(element, params);
        }
    }

    /// 단일 요소와 그 자식들에서 Variable 치환
    fn substitute_variable_in_element(
        element: &mut SevenMarkElement,
        params: &HashMap<String, String>,
    ) {
        match element {
            SevenMarkElement::Variable(var) => {
                // Parameter에 해당 변수가 있으면 Text로 치환
                if let Some(value) = params.get(&var.content) {
                    *element = SevenMarkElement::Text(TextElement {
                        location: Location::synthesized(),
                        content: value.clone(),
                    });
                }
            }
            _ => {
                // 자식 요소들도 재귀 처리
                element.traverse_children(&mut |child| {
                    Self::substitute_variable_in_element(child, params);
                });
            }
        }
    }

    /// Plain text 추출
    fn extract_plain_text(elements: &[SevenMarkElement]) -> String {
        elements
            .iter()
            .filter_map(|element| match element {
                SevenMarkElement::Text(text_element) => Some(text_element.content.as_str()),
                SevenMarkElement::Escape(escape_element) => Some(escape_element.content.as_str()),
                _ => None,
            })
            .collect::<String>()
    }
}
