use crate::SevenMarkElement;
use crate::sevenmark::{Location, TextElement, Traversable};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::sevenmark::processor::wiki::DocumentNamespace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeInfo {
    pub title: String,
    pub namespace: DocumentNamespace,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreprocessInfo {
    pub includes: HashMap<String, IncludeInfo>, // key: "namespace:title"
    pub categories: HashSet<String>,
    pub redirect: Option<String>,
    pub media: HashSet<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncludeContentInfo {
    pub includes: HashMap<String, IncludeInfo>,
    pub media: HashSet<String>,
}

impl Default for IncludeContentInfo {
    fn default() -> Self {
        Self {
            includes: HashMap::new(),
            media: HashSet::new(),
        }
    }
}

impl Default for PreprocessInfo {
    fn default() -> Self {
        Self {
            includes: HashMap::new(),
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
    /// namespace 문자열을 DocumentNamespace enum으로 변환
    fn parse_namespace(namespace: &str) -> DocumentNamespace {
        match namespace {
            "Document" => DocumentNamespace::Document,
            "User" => DocumentNamespace::User,
            "Template" => DocumentNamespace::Template,
            "File" => DocumentNamespace::File,
            "Category" => DocumentNamespace::Category,
            "Wiki" => DocumentNamespace::Wiki,
            _ => DocumentNamespace::Document, // 기본값
        }
    }

    /// namespace enum을 문자열로 변환
    fn namespace_to_string(namespace: &DocumentNamespace) -> &'static str {
        match namespace {
            DocumentNamespace::Document => "Document",
            DocumentNamespace::User => "User",
            DocumentNamespace::Template => "Template",
            DocumentNamespace::File => "File",
            DocumentNamespace::Category => "Category",
            DocumentNamespace::Wiki => "Wiki",
        }
    }

    /// 1단계: AST에서 Variable들을 해결
    fn substitute_all_variables_in_ast(elements: &mut [SevenMarkElement]) {
        let mut defined_vars = HashMap::new();

        for element in elements {
            Self::substitute_variables_in_element_recursively(element, &mut defined_vars);
        }
    }

    /// 재귀적으로 요소 처리 (mutable, forward-only)
    fn substitute_variables_in_element_recursively(
        element: &mut SevenMarkElement,
        defined_vars: &mut HashMap<String, String>,
    ) {
        match element {
            SevenMarkElement::DefineElement(def) => {
                // parameter들을 변수로 수집 (치환 후 저장)
                for (key, param) in &def.parameters {
                    let mut resolved_value = String::new();
                    Self::extract_text_with_variable_substitution(
                        &param.value,
                        defined_vars,
                        &mut resolved_value,
                    );
                    if !resolved_value.is_empty() {
                        defined_vars.insert(key.clone(), resolved_value);
                    }
                }
            }
            SevenMarkElement::Variable(var) => {
                // 정의된 변수가 있으면 Text로 치환
                if let Some(value) = defined_vars.get(&var.content) {
                    *element = SevenMarkElement::Text(TextElement {
                        location: Location::synthesized(),
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
    fn extract_text_with_variable_substitution(
        elements: &[SevenMarkElement],
        defined_vars: &HashMap<String, String>,
        result: &mut String,
    ) {
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
                    }
                }
                _ => {}
            }
        }
    }

    fn traverse_elements_and_collect_preprocess_info(
        elements: &mut [SevenMarkElement],
        info: &mut PreprocessInfo,
    ) {
        for element in elements {
            Self::extract_preprocess_info_from_element(element, info);
        }
    }

    /// Include, Media만 수집 (Category/Redirect 제외)
    pub fn collect_includes_and_media(elements: &[SevenMarkElement]) -> IncludeContentInfo {
        let mut elements_mut = elements.to_vec();
        let mut info = IncludeContentInfo::default();
        for element in &mut elements_mut {
            Self::extract_include_content_info_from_element(element, &mut info);
        }
        info
    }

    fn extract_include_content_info_from_element(
        element: &mut SevenMarkElement,
        info: &mut IncludeContentInfo,
    ) {
        match element {
            SevenMarkElement::Include(e) => {
                let title = Self::extract_plain_text(&e.content);
                if !title.is_empty() {
                    let parameters: HashMap<String, String> = e
                        .parameters
                        .iter()
                        .map(|(k, v)| (k.clone(), Self::extract_plain_text(&v.value)))
                        .collect();

                    // namespace 추출 및 enum 변환 (기본값: "Document")
                    let namespace_str = parameters
                        .get("namespace")
                        .map(|s| s.as_str())
                        .unwrap_or("Document");
                    let namespace = Self::parse_namespace(namespace_str);

                    let key = format!("{}:{}", Self::namespace_to_string(&namespace), title);

                    info.includes.insert(
                        key,
                        IncludeInfo {
                            title,
                            namespace,
                            parameters,
                        },
                    );
                }
            }
            SevenMarkElement::MediaElement(e) => {
                if let Some(url_param) = e.parameters.get("url") {
                    let url = Self::extract_plain_text(&url_param.value);
                    if !url.is_empty() {
                        info.media.insert(url);
                    }
                }
                if let Some(file_param) = e.parameters.get("file") {
                    let file = Self::extract_plain_text(&file_param.value);
                    if !file.is_empty() {
                        info.media.insert(file);
                    }
                }
            }
            _ => {}
        }

        // 자식 요소들도 재귀 처리
        element.traverse_children(&mut |child| {
            Self::extract_include_content_info_from_element(child, info);
        });
    }

    fn extract_preprocess_info_from_element(
        element: &mut SevenMarkElement,
        info: &mut PreprocessInfo,
    ) {
        match element {
            SevenMarkElement::Include(e) => {
                let title = Self::extract_plain_text(&e.content);
                if !title.is_empty() {
                    // parameters
                    let parameters: HashMap<String, String> = e
                        .parameters
                        .iter()
                        .map(|(k, v)| (k.clone(), Self::extract_plain_text(&v.value)))
                        .collect();

                    // namespace 추출 및 enum 변환 (기본값: "Document")
                    let namespace_str = parameters
                        .get("namespace")
                        .map(|s| s.as_str())
                        .unwrap_or("Document");
                    let namespace = Self::parse_namespace(namespace_str);

                    let key = format!("{}:{}", Self::namespace_to_string(&namespace), title);

                    info.includes.insert(
                        key,
                        IncludeInfo {
                            title,
                            namespace,
                            parameters,
                        },
                    );
                }
            }
            SevenMarkElement::Category(e) => {
                let name = Self::extract_plain_text(&e.content);
                if !name.is_empty() {
                    info.categories.insert(name);
                }
            }
            SevenMarkElement::Redirect(e) => {
                let target = Self::extract_plain_text(&e.content);
                if !target.is_empty() {
                    info.redirect = Some(target);
                }
            }
            SevenMarkElement::MediaElement(e) => {
                if let Some(url_param) = e.parameters.get("url") {
                    let url = Self::extract_plain_text(&url_param.value);
                    if !url.is_empty() {
                        info.media.insert(url);
                    }
                }
                if let Some(file_param) = e.parameters.get("file") {
                    let file = Self::extract_plain_text(&file_param.value);
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
