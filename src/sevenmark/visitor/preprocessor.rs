use crate::SevenMarkElement;
use serde::Serialize;
use std::collections::HashSet;

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
            match element {
                SevenMarkElement::Include(include_element) => {
                    let name = Self::extract_text_content(&include_element.content);
                    if !name.is_empty() {
                        info.includes.insert(name);
                    }
                }
                SevenMarkElement::Category(category_element) => {
                    let name = Self::extract_text_content(&category_element.content);
                    if !name.is_empty() {
                        info.categories.insert(name);
                    }
                }
                SevenMarkElement::Redirect(redirect_element) => {
                    let target = Self::extract_text_content(&redirect_element.content);
                    if !target.is_empty() {
                        info.redirect = Some(target);
                    }
                }
                SevenMarkElement::MediaElement(media_element) => {
                    let url = Self::extract_text_content(&media_element.url);
                    if !url.is_empty() {
                        info.media.insert(url);
                    }
                }
                _ => {}
            }
            Self::visit_nested_elements(element, info);
        }
    }

    fn visit_nested_elements(element: &SevenMarkElement, info: &mut PreprocessInfo) {
        match element {
            SevenMarkElement::StyledElement(styled) => {
                Self::collect_from_elements(&styled.content, info);
            }
            SevenMarkElement::TableElement(table) => {
                for row in &table.content {
                    for cell in &row.inner_content {
                        Self::collect_from_elements(&cell.content, info);
                    }
                }
            }
            SevenMarkElement::ListElement(list) => {
                for item in &list.content {
                    Self::collect_from_elements(&item.content, info);
                }
            }
            SevenMarkElement::FoldElement(fold) => {
                Self::collect_from_elements(&fold.content.0.content, info);
                Self::collect_from_elements(&fold.content.1.content, info);
            }
            SevenMarkElement::BlockQuoteElement(quote) => {
                Self::collect_from_elements(&quote.content, info);
            }
            SevenMarkElement::RubyElement(ruby) => {
                Self::collect_from_elements(&ruby.base, info);
                Self::collect_from_elements(&ruby.ruby, info);
            }
            SevenMarkElement::FootnoteElement(footnote) => {
                Self::collect_from_elements(&footnote.content, info);
            }
            SevenMarkElement::Header(header) => {
                Self::collect_from_elements(&header.content, info);
            }
            SevenMarkElement::BoldItalic(style)
            | SevenMarkElement::Bold(style)
            | SevenMarkElement::Italic(style)
            | SevenMarkElement::Strikethrough(style)
            | SevenMarkElement::Underline(style)
            | SevenMarkElement::Superscript(style)
            | SevenMarkElement::Subscript(style) => {
                Self::collect_from_elements(&style.content, info);
            }
            _ => {} // 다른 요소들은 순회하지 않음 (단순 요소 또는 순회 불필요)
        }
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
