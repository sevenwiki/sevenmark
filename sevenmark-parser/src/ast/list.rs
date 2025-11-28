use serde::Serialize;

use super::{Expression, Location, Parameters, SevenMarkElement};

/// 리스트 아이템
#[derive(Debug, Clone, Serialize)]
pub struct ListInnerElement1 {
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 리스트 콘텐츠 아이템 (아이템 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum ListContentItem {
    Item(ListInnerElement1),
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        items: Vec<ListInnerElement1>,
    },
}

/// 리스트 요소
#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub kind: String,
    pub parameters: Parameters,
    pub content: Vec<ListContentItem>,
}