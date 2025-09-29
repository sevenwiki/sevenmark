use crate::{SevenMarkElement};
use serde::Serialize;
use std::collections::HashSet;
use crate::sevenmark::Traversable;

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
    fn collect_info(elements: &[SevenMarkElement]) -> PreprocessInfo;
}

pub struct SevenMarkPreprocessor;

impl PreVisitor for SevenMarkPreprocessor {
    fn collect_info(elements: &[SevenMarkElement]) -> PreprocessInfo {
        let mut info = PreprocessInfo::default();
        Self::collect_from_elements(elements, &mut info);
        info
    }
}

impl SevenMarkPreprocessor {
    fn collect_from_elements(elements: &[SevenMarkElement], info: &mut PreprocessInfo) {
        for element in elements {
            Self::visit_element(element, info);
        }
    }

    fn visit_element(element: &SevenMarkElement, info: &mut PreprocessInfo) {
        // 특수 케이스 처리 (정보 수집이 필요한 4개 요소만)
        match element {
            SevenMarkElement::Include(e) => {
                let name = Self::extract_text_content(&e.content);
                if !name.is_empty() {
                    info.includes.insert(name);
                }
            }
            SevenMarkElement::Category(e) => {
                let name = Self::extract_text_content(&e.content);
                if !name.is_empty() {
                    info.categories.insert(name);
                }
            }
            SevenMarkElement::Redirect(e) => {
                let target = Self::extract_text_content(&e.content);
                if !target.is_empty() {
                    info.redirect = Some(target);
                }
            }
            SevenMarkElement::MediaElement(e) => {
                if let Some(url_param) = e.parameters.get("url") {
                    let url = Self::extract_text_content(&url_param.value);
                    if !url.is_empty() {
                        info.media.insert(url);
                    }
                }
                if let Some(file_param) = e.parameters.get("file") {
                    let file = Self::extract_text_content(&file_param.value);
                    if !file.is_empty() {
                        info.media.insert(file);
                    }
                }
            }
            _ => {}
        }

        // trait을 사용한 자동 순회
        element.traverse_children(&mut |child| {
            Self::visit_element(child, info);
        });
    }
    fn extract_text_content(elements: &[SevenMarkElement]) -> String {
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
