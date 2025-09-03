pub mod sevenmark;

pub use sevenmark::ast::SevenMarkElement;
pub use sevenmark::core::parse_document;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_sevenmark_to_json(input: &str) -> String {
    let elements = parse_document(input);
    serde_json::to_string(&elements).unwrap_or_else(|_| "[]".to_string())
}
