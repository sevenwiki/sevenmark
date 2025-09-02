use crate::SevenMarkElement;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct PreprocessInfo {
    pub includes: HashSet<String>,
    pub categories: Vec<String>,
    pub redirect: Option<String>,
    pub media: HashSet<String>,
}

impl Default for PreprocessInfo {
    fn default() -> Self {
        Self {
            includes: HashSet::new(),
            categories: Vec::new(),
            redirect: None,
            media: HashSet::new(),
        }
    }
}

pub trait PreVisitor {
    fn collect_info(elements: &[SevenMarkElement]) -> PreprocessInfo;
}
