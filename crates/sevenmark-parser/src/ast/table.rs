use serde::Serialize;

use super::{Expression, Location, Parameters, SevenMarkElement};

/// 테이블 셀
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement2 {
    pub parameters: Parameters,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub x: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub y: Vec<SevenMarkElement>,
    pub content: Vec<SevenMarkElement>,
}

/// 테이블 셀 콘텐츠 아이템 (셀 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum TableCellItem {
    Cell(TableInnerElement2),
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        cells: Vec<TableInnerElement2>,
    },
}

/// 테이블 행
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement1 {
    pub parameters: Parameters,
    pub content: Vec<TableCellItem>,
}

/// 테이블 행 콘텐츠 아이템 (행 또는 조건부)
#[derive(Debug, Clone, Serialize)]
pub enum TableRowItem {
    Row(TableInnerElement1),
    Conditional {
        #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
        location: Location,
        condition: Expression,
        rows: Vec<TableInnerElement1>,
    },
}

/// 테이블 요소
#[derive(Debug, Clone, Serialize)]
pub struct TableElement {
    #[cfg_attr(not(feature = "include_locations"), serde(skip_serializing))]
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<TableRowItem>,
}
