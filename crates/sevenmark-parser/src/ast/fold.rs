use crate::ast::{AstNode, Parameters};
use serde::Serialize;

/// Fold 내부 요소 (title 또는 content)
#[derive(Debug, Clone, Serialize)]
pub struct FoldInnerElement {
    pub parameters: Parameters,
    pub children: Vec<AstNode>,
}
