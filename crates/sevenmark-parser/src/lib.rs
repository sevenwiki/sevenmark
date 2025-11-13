pub mod ast;
pub mod context;
pub mod core;
pub mod error;
pub mod parser;

// Re-export commonly used types
pub use ast::*;
pub use context::*;
pub use core::parse_document;
pub use error::*;
pub use parser::{InputSource, ParserInput};

// WASM bindings
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_sevenmark_wasm(input: &str) -> String {
    let elements = parse_document(input);
    serde_json::to_string(&elements).unwrap_or_else(|_| "[]".to_string())
}
