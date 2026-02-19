use std::cell::RefCell;

use sevenmark_lsp_core::server_state::LspState;
use wasm_bindgen::prelude::*;

/// Parse sevenmark to AST with UTF-16 absolute offsets (for CodeMirror 6)
#[wasm_bindgen]
pub fn parse_sevenmark_to_codemirror(input: &str) -> String {
    use sevenmark_parser::core::parse_document;
    use sevenmark_utils::convert_ast_to_utf16_offset_json;

    let elements = parse_document(input);
    convert_ast_to_utf16_offset_json(&elements, input)
}

/// Parse sevenmark to AST with byte offsets (for section editing)
#[wasm_bindgen]
pub fn parse_sevenmark(input: &str) -> String {
    use sevenmark_parser::core::parse_document;

    let elements = parse_document(input);
    serde_json::to_string(&elements).unwrap_or_else(|e| format!(r#"{{"error":"{}"}}"#, e))
}

thread_local! {
    static LSP_STATE: RefCell<LspState> = RefCell::new(LspState::new());
}

/// Process a JSON-RPC message for the LSP and return a JSON response.
///
/// Returns a JSON object with:
/// - `response`: the JSON-RPC response string (if the message was a request)
/// - `notifications`: array of JSON-RPC notification strings (e.g. publishDiagnostics)
#[wasm_bindgen]
pub fn handle_lsp_message(json: &str) -> String {
    LSP_STATE.with(|s| {
        let result = s.borrow_mut().handle_message(json);
        serde_json::to_string(&result).unwrap()
    })
}
