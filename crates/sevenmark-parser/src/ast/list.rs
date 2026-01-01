use serde::Serialize;

use super::{AstNode, Location, Parameters};

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