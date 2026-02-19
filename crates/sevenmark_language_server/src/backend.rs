use dashmap::DashMap;
use sevenmark_parser::ast::{Element, Traversable};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

use crate::diagnostics::collect_diagnostics;
use crate::document::DocumentState;
use crate::folding::collect_folding_ranges;

pub struct Backend {
    pub client: Client,
    pub documents: DashMap<String, DocumentState>,
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
                document_symbol_provider: Some(OneOf::Left(true)),
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
        let text = params.text_document.text;
        self.on_change(uri, text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            self.on_change(params.text_document.uri, change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.remove(&uri.to_string());
        self.client
            .publish_diagnostics(uri, Vec::new(), None)
            .await;
    }

    async fn folding_range(
        &self,
        params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>> {
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
    async fn on_change(&self, uri: Uri, text: String) {
        let uri_key = uri.to_string();
        let state = DocumentState::new(text);
        let diagnostics = collect_diagnostics(&state);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
        self.documents.insert(uri_key, state);
    }
}

/// Extracts document symbols (headers and variable definitions) from the AST.
fn collect_document_symbols(state: &DocumentState) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    visit_for_symbols(&state.elements, state, &mut symbols);
    symbols
}

/// Recursively walks elements collecting symbols.
fn visit_for_symbols(
    elements: &[Element],
    state: &DocumentState,
    symbols: &mut Vec<DocumentSymbol>,
) {
    for element in elements {
        match element {
            Element::Header(h) => {
                let name = extract_text_from_children(&h.children);
                let name = if name.is_empty() {
                    format!("Header (level {})", h.level)
                } else {
                    name
                };
                let (start, end) = state.line_index.span_to_range(&state.text, &h.span);
                let range = Range::new(
                    Position::new(start.0, start.1),
                    Position::new(end.0, end.1),
                );

                #[allow(deprecated)]
                symbols.push(DocumentSymbol {
                    name,
                    detail: Some(format!("Level {}", h.level)),
                    kind: SymbolKind::STRING,
                    range,
                    selection_range: range,
                    children: None,
                    tags: None,
                    deprecated: None,
                });

                visit_for_symbols(&h.children, state, symbols);
            }
            Element::Define(d) => {
                if let Some(name_param) = d.parameters.get("name") {
                    let var_name = extract_text_from_children(&name_param.value);
                    if !var_name.is_empty() {
                        let (start, end) =
                            state.line_index.span_to_range(&state.text, &d.span);
                        let range = Range::new(
                            Position::new(start.0, start.1),
                            Position::new(end.0, end.1),
                        );

                        #[allow(deprecated)]
                        symbols.push(DocumentSymbol {
                            name: var_name,
                            detail: Some("Define".to_string()),
                            kind: SymbolKind::VARIABLE,
                            range,
                            selection_range: range,
                            children: None,
                            tags: None,
                            deprecated: None,
                        });
                    }
                }
            }
            other => {
                other.traverse_children_ref(&mut |child| {
                    visit_for_symbols(std::slice::from_ref(child), state, symbols);
                });
            }
        }
    }
}

/// Extracts plain text content from child elements for display.
fn extract_text_from_children(elements: &[Element]) -> String {
    let mut result = String::new();
    for element in elements {
        match element {
            Element::Text(t) => result.push_str(&t.value),
            Element::Escape(e) => result.push_str(&e.value),
            _ => {}
        }
    }
    result.trim().to_string()
}