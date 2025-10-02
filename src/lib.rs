#[cfg(feature = "server")]
pub mod api;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "server")]
pub mod connection;
#[cfg(feature = "server")]
pub mod errors;
pub mod sevenmark;
#[cfg(feature = "server")]
pub mod state;
#[cfg(feature = "server")]
pub mod utils;

pub use sevenmark::ast::SevenMarkElement;
pub use sevenmark::core::parse_document;
pub use sevenmark::processor::convert_ast_to_monaco_json;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_sevenmark_to_monaco(input: &str) -> String {
    let elements = parse_document(input);
    convert_ast_to_monaco_json(&elements, input)
}
