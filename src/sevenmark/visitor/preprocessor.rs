use crate::SevenMarkElement;
use crate::sevenmark::{TextElement, Traversable};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize)]
pub struct PreprocessInfo {
    pub includes: HashSet<String>,
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub media: HashSet<String>,
}

impl Default for PreprocessInfo {
    fn default() -> Self {
        Self {
            includes: HashSet::new(),
            categories: HashSet::new(),
            redirect: None,
            media: HashSet::new(),
        }
    }
}

pub trait PreVisitor {
    fn preprocess(elements: &[SevenMarkElement]) -> PreprocessInfo;
}

pub struct SevenMarkPreprocessor;

impl PreVisitor for SevenMarkPreprocessor {
    fn preprocess(elements: &[SevenMarkElement]) -> PreprocessInfo {
        let mut elements_mut = elements.to_vec();

        // 1단계: Define 수집 & Variable 치환 (forward-only)
        Self::substitute_all_variables_in_ast(&mut elements_mut);

        // 2단계: 정보 수집
        let mut info = PreprocessInfo::default();
        Self::traverse_elements_and_collect_preprocess_info(&mut elements_mut, &mut info);
        info
    }
}

impl SevenMarkPreprocessor {
    /// 1단계: AST에서 Variable들을 해결
    fn substitute_all_variables_in_ast(elements: &mut [SevenMarkElement]) {
        let mut defined_vars = HashMap::new();

        for element in elements {
            Self::substitute_variables_in_element_recursively(element, &mut defined_vars);
        }
    }

    /// 재귀적으로 요소 처리 (mutable, forward-only)
    fn substitute_variables_in_element_recursively(element: &mut SevenMarkElement, defined_vars: &mut HashMap<String, String>) {
        match element {
            SevenMarkElement::DefineElement(def) => {
                // parameter들을 변수로 수집 (치환 후 저장)
                for (key, param) in &def.parameters {
                    let mut resolved_value = String::new();
                    Self::extract_text_with_variable_substitution(&param.value, defined_vars, &mut resolved_value);
                    if !resolved_value.is_empty() {
                        defined_vars.insert(key.clone(), resolved_value);
                    }
                }
            }
            SevenMarkElement::Variable(var) => {
                // 정의된 변수가 있으면 Text로 치환
                if let Some(value) = defined_vars.get(&var.content) {
                    *element = SevenMarkElement::Text(TextElement {
                        location: var.location.clone(),
                        content: value.clone(),
                    });
                }
            }
            _ => {}
        }

        // 자식 요소들 재귀 처리 (mutable)
        element.traverse_children(&mut |child| {
            Self::substitute_variables_in_element_recursively(child, defined_vars);
        });
    }

    /// Vec<SevenMarkElement>에서 텍스트 추출하면서 Variable 해결
    fn extract_text_with_variable_substitution(elements: &[SevenMarkElement], defined_vars: &HashMap<String, String>, result: &mut String) {
        for element in elements {
            match element {
                SevenMarkElement::Text(text_element) => {
                    result.push_str(&text_element.content);
                }
                SevenMarkElement::Escape(escape_element) => {
                    result.push_str(&escape_element.content);
                }
                SevenMarkElement::Variable(var_element) => {
                    if let Some(value) = defined_vars.get(&var_element.content) {
                        result.push_str(value);
                    } else {
                        // 정의되지 않은 변수는 원래 형태로 유지
                        result.push_str(&format!("[var({})]", var_element.content));
                    }
                }
                _ => {
                }
            }
        }
    }

    fn traverse_elements_and_collect_preprocess_info(elements: &mut [SevenMarkElement], info: &mut PreprocessInfo) {
        for element in elements {
            Self::extract_preprocess_info_from_element(element, info);
        }
    }

    fn extract_preprocess_info_from_element(element: &mut SevenMarkElement, info: &mut PreprocessInfo) {
        // 특수 케이스 처리 (정보 수집이 필요한 5개 요소)
        match element {
            SevenMarkElement::Include(e) => {
                let name = Self::extract_plain_text_ignoring_markup(&e.content);
                if !name.is_empty() {
                    info.includes.insert(name);
                }
            }
            SevenMarkElement::Category(e) => {
                let name = Self::extract_plain_text_ignoring_markup(&e.content);
                if !name.is_empty() {
                    info.categories.insert(name);
                }
            }
            SevenMarkElement::Redirect(e) => {
                let target = Self::extract_plain_text_ignoring_markup(&e.content);
                if !target.is_empty() {
                    info.redirect = Some(target);
                }
            }
            SevenMarkElement::MediaElement(e) => {
                if let Some(url_param) = e.parameters.get("url") {
                    let url = Self::extract_plain_text_ignoring_markup(&url_param.value);
                    if !url.is_empty() {
                        info.media.insert(url);
                    }
                }
                if let Some(file_param) = e.parameters.get("file") {
                    let file = Self::extract_plain_text_ignoring_markup(&file_param.value);
                    if !file.is_empty() {
                        info.media.insert(file);
                    }
                }
            }
            _ => {}
        }

        // trait을 사용한 자동 순회
        element.traverse_children(&mut |child| {
            Self::extract_preprocess_info_from_element(child, info);
        });
    }
    fn extract_plain_text_ignoring_markup(elements: &[SevenMarkElement]) -> String {
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
