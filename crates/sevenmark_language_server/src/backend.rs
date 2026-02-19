use dashmap::DashMap;
use sevenmark_lsp_core::completion::get_completions;
use sevenmark_lsp_core::definition::find_definition;
use sevenmark_lsp_core::diagnostics::collect_diagnostics;
use sevenmark_lsp_core::document::DocumentState;
use sevenmark_lsp_core::folding::collect_folding_ranges;
use sevenmark_lsp_core::hover::get_hover;
use sevenmark_lsp_core::semantic_tokens::{collect_semantic_tokens, legend};
use sevenmark_lsp_core::symbols::collect_document_symbols;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
    pub client: Client,
    pub documents: DashMap<String, DocumentState>,
    pub document_versions: DashMap<String, i32>,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
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
            },
            server_info: Some(ServerInfo {
                name: "sevenmark-language-server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        let text = params.text_document.text;
        self.on_change(uri, Some(version), text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let version = params.text_document.version;
        let mut changes = params.content_changes;
        if changes.is_empty() {
            return;
        }

        if changes.len() != 1 {
            self.client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "Expected one FULL sync change event, got {}. Using last change.",
                        changes.len()
                    ),
                )
                .await;
        }

        if let Some(change) = changes.pop() {
            self.on_change(params.text_document.uri, Some(version), change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let uri_key = uri.to_string();
        self.documents.remove(&uri_key);
        self.document_versions.remove(&uri_key);
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let uri_key = uri.to_string();
        let Some(state) = self.documents.get(&uri_key) else {
            return Ok(None);
        };
        let byte_offset =
            state
                .line_index
                .position_to_byte_offset(&state.text, pos.line, pos.character);
        let location = find_definition(&state, &uri, byte_offset);
        Ok(location.map(GotoDefinitionResponse::Scalar))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let uri_key = uri.to_string();
        let Some(state) = self.documents.get(&uri_key) else {
            return Ok(None);
        };
        let byte_offset =
            state
                .line_index
                .position_to_byte_offset(&state.text, pos.line, pos.character);
        Ok(get_hover(&state, byte_offset))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let pos = params.text_document_position.position;
        let Some(state) = self.documents.get(&uri) else {
            return Ok(None);
        };
        let byte_offset =
            state
                .line_index
                .position_to_byte_offset(&state.text, pos.line, pos.character);
        let items = get_completions(&state, pos, byte_offset);
        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(items)))
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return Ok(None);
        };
        let tokens = collect_semantic_tokens(&state);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return Ok(None);
        };
        let ranges = collect_folding_ranges(&state);
        if ranges.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ranges))
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        let Some(state) = self.documents.get(&uri) else {
            return Ok(None);
        };
        let symbols = collect_document_symbols(&state);
        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    }
}

impl Backend {
    /// Parses the document, publishes diagnostics, and caches state.
    async fn on_change(&self, uri: Uri, version: Option<i32>, text: String) {
        let uri_key = uri.to_string();

        if let Some(version) = version {
            if let Some(prev_version) = self.document_versions.get(&uri_key)
                && version < *prev_version
            {
                return;
            }
        }

        let state = DocumentState::new(text);
        let diagnostics = collect_diagnostics(&state);

        // Cache first so hover/completion/definition always see latest parse.
        self.documents.insert(uri_key.clone(), state);
        if let Some(version) = version {
            self.document_versions.insert(uri_key.clone(), version);
        }

        self.client
            .publish_diagnostics(uri, diagnostics, version)
            .await;
    }
}
