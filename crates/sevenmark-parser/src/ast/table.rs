use serde::Serialize;

use super::{AstNode, Expression, Location, Parameters};

/// 테이블 행 콘텐츠 아이템 (행 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum TableRowChild {
    /// 일반 행
    Row(TableRow),
    /// 조건부 행 ({{{#if condition :: [[row]]...}}})
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        children: Vec<TableRow>,
    },
}

/// 테이블 셀 콘텐츠 아이템 (셀 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum TableCellChild {
    /// 일반 셀
    Cell(TableCell),
    /// 조건부 셀 ({{{#if condition :: [[cell]]...}}})
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        children: Vec<TableCell>,
    },
}

/// 테이블 행 (location 포함)
#[derive(Debug, Clone, Serialize)]
pub struct TableRow {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub children: Vec<TableCellChild>,
}

impl TableRow {
    pub fn new(location: Location, parameters: Parameters, children: Vec<TableCellChild>) -> Self {
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