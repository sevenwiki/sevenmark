pub mod position_converter;

#[cfg(feature = "server")]
pub mod expression_evaluator;
#[cfg(feature = "server")]
pub mod postprocessor;
#[cfg(feature = "server")]
pub mod preprocessor;
#[cfg(feature = "server")]
pub mod processor;
#[cfg(feature = "server")]
pub mod wiki;

pub use position_converter::*;

#[cfg(feature = "server")]
pub use postprocessor::*;
#[cfg(feature = "server")]
pub use preprocessor::*;
#[cfg(feature = "server")]
pub use processor::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_sevenmark_to_monaco(input: &str) -> String {
    use sevenmark_parser::core::parse_document;

    let elements = parse_document(input);
    convert_ast_to_line_column_json(&elements, input)
}
