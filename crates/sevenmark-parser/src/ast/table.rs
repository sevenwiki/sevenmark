use serde::Serialize;

use super::{AstNode, Location, Parameters};

/// 테이블 행 (location 포함)
#[derive(Debug, Clone, Serialize)]
pub struct TableRow {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub children: Vec<TableCell>,
}

impl TableRow {
    pub fn new(location: Location, parameters: Parameters, children: Vec<TableCell>) -> Self {
        Self {
            location,
            parameters,
            children,
        }
    }
}

/// 테이블 셀 (location 포함)
#[derive(Debug, Clone, Serialize)]
pub struct TableCell {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub x: Vec<AstNode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub y: Vec<AstNode>,
    pub children: Vec<AstNode>,
}

impl TableCell {
    pub fn new(
        location: Location,
        parameters: Parameters,
        x: Vec<AstNode>,
        y: Vec<AstNode>,
        children: Vec<AstNode>,
    ) -> Self {
        Self {
            location,
            parameters,
            x,
            y,
            children,
        }
    }
}