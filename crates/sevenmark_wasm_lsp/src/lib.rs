use std::cell::RefCell;

use sevenmark_lsp_core::server_state::LspState;
use wasm_bindgen::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::handle_lsp_message;

    #[test]
    fn initialize_returns_json_rpc_result() {
        let response =
            handle_lsp_message(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#);
        let value: serde_json::Value = serde_json::from_str(&response).unwrap();

        assert!(value.get("response").is_some());
        assert!(value.get("notifications").is_some());
    }
}
