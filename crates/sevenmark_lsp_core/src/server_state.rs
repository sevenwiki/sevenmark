use std::collections::HashMap;

use ls_types::*;
use serde_json::Value;

use crate::completion::get_completions;
use crate::definition::find_definition;
use crate::diagnostics::collect_diagnostics;
use crate::document::DocumentState;
use crate::folding::collect_folding_ranges;
use crate::hover::get_hover;
use crate::semantic_tokens::{collect_semantic_tokens, legend};
use crate::symbols::collect_document_symbols;

/// Transport-agnostic LSP state. Owns all open documents and handles
/// JSON-RPC messages synchronously - no async runtime required.
pub struct LspState {
    documents: HashMap<String, DocumentState>,
    document_versions: HashMap<String, i32>,
}

/// The result of handling a single JSON-RPC message.
#[derive(serde::Serialize)]
pub struct HandleResult {
    /// JSON-RPC response (present only when the incoming message was a *request* with an `id`).
    pub response: Option<String>,
    /// Server → client push notifications (e.g. `textDocument/publishDiagnostics`).
    pub notifications: Vec<String>,
}

impl LspState {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            document_versions: HashMap::new(),
        }
    }

    /// Processes a single JSON-RPC message (request or notification) and returns
    /// the response plus any server-initiated notifications.
    pub fn handle_message(&mut self, json: &str) -> HandleResult {
        let msg: Value = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                return HandleResult {
                    response: Some(make_error_response(
                        Value::Null,
                        -32700,
                        &format!("Parse error: {e}"),
                    )),
                    notifications: Vec::new(),
                };
            }
        };

        let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = msg.get("id").cloned();
        let params = msg.get("params").cloned().unwrap_or(Value::Null);

        match method {
            "initialize" => self.handle_initialize(id, params),
            "initialized" => HandleResult::empty(),
            "shutdown" => HandleResult::response_only(id, Value::Null),
            "textDocument/didOpen" => self.handle_did_open(params),
            "textDocument/didChange" => self.handle_did_change(params),
            "textDocument/didClose" => self.handle_did_close(params),
            "textDocument/completion" => self.handle_completion(id, params),
            "textDocument/hover" => self.handle_hover(id, params),
            "textDocument/definition" => self.handle_definition(id, params),
            "textDocument/semanticTokens/full" => self.handle_semantic_tokens(id, params),
            "textDocument/foldingRange" => self.handle_folding_range(id, params),
            "textDocument/documentSymbol" => self.handle_document_symbol(id, params),
            _ => {
                if let Some(id) = id {
                    // Unknown request → method not found
                    HandleResult {
                        response: Some(make_error_response(
                            id,
                            -32601,
                            &format!("Method not found: {method}"),
                        )),
                        notifications: Vec::new(),
                    }
                } else {
                    // Unknown notification → silently ignore
                    HandleResult::empty()
                }
            }
        }
    }

    // ── Request handlers ─────────────────────────────────────────────────

    fn handle_initialize(&self, id: Option<Value>, _params: Value) -> HandleResult {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::FULL),
                    ..Default::default()
                },
            )),
            completion_provider: Some(CompletionOptions {
                trigger_characters: Some(vec![
                    "[".to_string(),
                    "#".to_string(),
                    "(".to_string(),
                ]),
                ..Default::default()
            }),
            definition_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        legend: legend(),
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        range: None,
                        ..Default::default()
                    },
                ),
            ),
            folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
            ..Default::default()
        };

        let result = InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "sevenmark-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            ..Default::default()
        };

        HandleResult::response_only(id, serde_json::to_value(result).unwrap())
    }

    fn handle_completion(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<CompletionParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document_position.text_document.uri.to_string();
        let pos = params.text_document_position.position;
        let Some(state) = self.documents.get(&uri) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let byte_offset = state
            .line_index
            .position_to_byte_offset(&state.text, pos.line, pos.character);
        let items = get_completions(state, pos, byte_offset);
        let result = if items.is_empty() {
            Value::Null
        } else {
            serde_json::to_value(items).unwrap()
        };
        HandleResult::response_only(id, result)
    }

    fn handle_hover(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<HoverParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let pos = params.text_document_position_params.position;
        let Some(state) = self.documents.get(&uri) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let byte_offset = state
            .line_index
            .position_to_byte_offset(&state.text, pos.line, pos.character);
        let result = match get_hover(state, byte_offset) {
            Some(hover) => serde_json::to_value(hover).unwrap(),
            None => Value::Null,
        };
        HandleResult::response_only(id, result)
    }

    fn handle_definition(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<GotoDefinitionParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document_position_params.text_document.uri.clone();
        let pos = params.text_document_position_params.position;
        let uri_key = uri.to_string();
        let Some(state) = self.documents.get(&uri_key) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let byte_offset = state
            .line_index
            .position_to_byte_offset(&state.text, pos.line, pos.character);
        let result = match find_definition(state, &uri, byte_offset) {
            Some(location) => serde_json::to_value(location).unwrap(),
            None => Value::Null,
        };
        HandleResult::response_only(id, result)
    }

    fn handle_semantic_tokens(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<SemanticTokensParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let tokens = collect_semantic_tokens(state);
        let result = SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        });
        HandleResult::response_only(id, serde_json::to_value(result).unwrap())
    }

    fn handle_folding_range(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<FoldingRangeParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let ranges = collect_folding_ranges(state);
        let result = if ranges.is_empty() {
            Value::Null
        } else {
            serde_json::to_value(ranges).unwrap()
        };
        HandleResult::response_only(id, result)
    }

    fn handle_document_symbol(&self, id: Option<Value>, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<DocumentSymbolParams>(params) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return HandleResult::response_only(id, Value::Null);
        };
        let symbols = collect_document_symbols(state);
        let result = if symbols.is_empty() {
            Value::Null
        } else {
            serde_json::to_value(symbols).unwrap()
        };
        HandleResult::response_only(id, result)
    }

    // ── Notification handlers ────────────────────────────────────────────

    fn handle_did_open(&mut self, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<DidOpenTextDocumentParams>(params) else {
            return HandleResult::empty();
        };
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        let text = params.text_document.text;
        self.on_change(uri, Some(version), text)
    }

    fn handle_did_change(&mut self, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<DidChangeTextDocumentParams>(params) else {
            return HandleResult::empty();
        };
        let version = params.text_document.version;
        let mut changes = params.content_changes;
        if changes.is_empty() {
            return HandleResult::empty();
        }
        if let Some(change) = changes.pop() {
            return self.on_change(params.text_document.uri, Some(version), change.text);
        }
        HandleResult::empty()
    }

    fn handle_did_close(&mut self, params: Value) -> HandleResult {
        let Ok(params) = serde_json::from_value::<DidCloseTextDocumentParams>(params) else {
            return HandleResult::empty();
        };
        let uri = params.text_document.uri;
        let uri_key = uri.to_string();
        self.documents.remove(&uri_key);
        self.document_versions.remove(&uri_key);

        // Push empty diagnostics to clear
        let notification = make_notification(
            "textDocument/publishDiagnostics",
            serde_json::to_value(PublishDiagnosticsParams {
                uri,
                diagnostics: Vec::new(),
                version: None,
            })
            .unwrap(),
        );

        HandleResult {
            response: None,
            notifications: vec![notification],
        }
    }

    // ── Internal helpers ─────────────────────────────────────────────────

    fn on_change(&mut self, uri: Uri, version: Option<i32>, text: String) -> HandleResult {
        let uri_key = uri.to_string();

        if let Some(version) = version {
            if let Some(&prev_version) = self.document_versions.get(&uri_key) {
                if version < prev_version {
                    return HandleResult::empty();
                }
            }
        }

        let state = DocumentState::new(text);
        let diagnostics = collect_diagnostics(&state);

        self.documents.insert(uri_key.clone(), state);
        if let Some(version) = version {
            self.document_versions.insert(uri_key, version);
        }

        let notification = make_notification(
            "textDocument/publishDiagnostics",
            serde_json::to_value(PublishDiagnosticsParams {
                uri,
                diagnostics,
                version,
            })
            .unwrap(),
        );

        HandleResult {
            response: None,
            notifications: vec![notification],
        }
    }
}

impl Default for LspState {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleResult {
    fn empty() -> Self {
        Self {
            response: None,
            notifications: Vec::new(),
        }
    }

    fn response_only(id: Option<Value>, result: Value) -> Self {
        let response = id.map(|id| {
            serde_json::to_string(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": result,
            }))
            .unwrap()
        });
        Self {
            response,
            notifications: Vec::new(),
        }
    }
}

fn make_error_response(id: Value, code: i64, message: &str) -> String {
    serde_json::to_string(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
        }
    }))
    .unwrap()
}

fn make_notification(method: &str, params: Value) -> String {
    serde_json::to_string(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_returns_capabilities() {
        let mut state = LspState::new();
        let msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let result = state.handle_message(msg);
        assert!(result.response.is_some());
        let resp: Value = serde_json::from_str(&result.response.unwrap()).unwrap();
        assert!(resp.get("result").is_some());
        let caps = &resp["result"]["capabilities"];
        assert!(caps["textDocumentSync"].is_object());
        assert!(caps["completionProvider"].is_object());
        assert!(caps["hoverProvider"].is_boolean());
    }

    #[test]
    fn did_open_publishes_diagnostics() {
        let mut state = LspState::new();
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///test.sm",
                    "languageId": "sevenmark",
                    "version": 1,
                    "text": "hello world"
                }
            }
        });
        let result = state.handle_message(&msg.to_string());
        assert!(result.response.is_none());
        assert_eq!(result.notifications.len(), 1);
        let notif: Value = serde_json::from_str(&result.notifications[0]).unwrap();
        assert_eq!(notif["method"], "textDocument/publishDiagnostics");
    }

    #[test]
    fn hover_on_bold() {
        let mut state = LspState::new();
        // Open document
        let open = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///test.sm",
                    "languageId": "sevenmark",
                    "version": 1,
                    "text": "**bold**"
                }
            }
        });
        state.handle_message(&open.to_string());

        // Hover request
        let hover = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": "file:///test.sm" },
                "position": { "line": 0, "character": 3 }
            }
        });
        let result = state.handle_message(&hover.to_string());
        assert!(result.response.is_some());
        let resp: Value = serde_json::from_str(&result.response.unwrap()).unwrap();
        let content = resp["result"]["contents"]["value"].as_str().unwrap();
        assert!(content.contains("Bold"));
    }

    #[test]
    fn unknown_method_returns_error() {
        let mut state = LspState::new();
        let msg = r#"{"jsonrpc":"2.0","id":1,"method":"foo/bar","params":{}}"#;
        let result = state.handle_message(msg);
        assert!(result.response.is_some());
        let resp: Value = serde_json::from_str(&result.response.unwrap()).unwrap();
        assert!(resp.get("error").is_some());
    }

    #[test]
    fn did_close_clears_diagnostics() {
        let mut state = LspState::new();
        // Open
        let open = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///test.sm",
                    "languageId": "sevenmark",
                    "version": 1,
                    "text": "[var(x)]"
                }
            }
        });
        state.handle_message(&open.to_string());

        // Close
        let close = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didClose",
            "params": {
                "textDocument": { "uri": "file:///test.sm" }
            }
        });
        let result = state.handle_message(&close.to_string());
        assert_eq!(result.notifications.len(), 1);
        let notif: Value = serde_json::from_str(&result.notifications[0]).unwrap();
        let diags = &notif["params"]["diagnostics"];
        assert!(diags.as_array().unwrap().is_empty());
    }
}