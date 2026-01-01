use serde::Serialize;

use super::{AstNode, Expression, Location, Parameters};

/// 리스트 아이템 콘텐츠 (아이템 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum ListItemChild {
    /// 일반 아이템
    Item(ListItem),
    /// 조건부 아이템 ({{{#if condition :: [[item]]...}}})
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        children: Vec<ListItem>,
    },
}

/// 리스트 아이템 (location 포함)
#[derive(Debug, Clone, Serialize)]
pub struct ListItem {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub children: Vec<AstNode>,
}

impl ListItem {
    pub fn new(location: Location, parameters: Parameters, children: Vec<AstNode>) -> Self {
        Self {
            location,
            parameters,
            children,
        }
    }
}