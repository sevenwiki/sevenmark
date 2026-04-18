use serde::{Serialize, Serializer};
use std::fmt;

use super::{Element, Expression, Parameters, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered,
    OrderedNumeric,
    OrderedAlphaLower,
    OrderedAlphaUpper,
    OrderedRomanLower,
    OrderedRomanUpper,
}

impl ListKind {
    pub fn as_code(self) -> &'static str {
        match self {
            ListKind::Unordered => "",
            ListKind::OrderedNumeric => "1",
            ListKind::OrderedAlphaLower => "a",
            ListKind::OrderedAlphaUpper => "A",
            ListKind::OrderedRomanLower => "i",
            ListKind::OrderedRomanUpper => "I",
        }
    }

    pub fn ordered_type_attr(self) -> Option<&'static str> {
        match self {
            ListKind::Unordered | ListKind::OrderedNumeric => None,
            ListKind::OrderedAlphaLower => Some("a"),
            ListKind::OrderedAlphaUpper => Some("A"),
            ListKind::OrderedRomanLower => Some("i"),
            ListKind::OrderedRomanUpper => Some("I"),
        }
    }

    pub fn is_ordered(self) -> bool {
        !matches!(self, ListKind::Unordered)
    }
}

impl fmt::Display for ListKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_code())
    }
}

impl Serialize for ListKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_code())
    }
}

/// 리스트 요소 {{{#list ...}}}
#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub kind: ListKind,
    pub parameters: Parameters,
    pub children: Vec<ListContentItem>,
}

/// 리스트 콘텐츠 아이템 (아이템 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum ListContentItem {
    Item(ListItemElement),
    Conditional(ConditionalListItems),
}

/// 리스트 아이템
#[derive(Debug, Clone, Serialize)]
pub struct ListItemElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub parameters: Parameters,
    pub children: Vec<Element>,
}

/// 조건부 리스트 아이템 ({{{#if condition :: [[item]]...}}})
#[derive(Debug, Clone, Serialize)]
pub struct ConditionalListItems {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub open_span: Span,
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub close_span: Span,
    pub condition: Expression,
    pub items: Vec<ListItemElement>,
}
