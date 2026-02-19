mod ast_walk;
mod backend;
mod completion;
mod definition;
mod diagnostics;
mod document;
mod folding;
mod hover;
mod semantic_tokens;

use dashmap::DashMap;
use tower_lsp_server::{LspService, Server};

use backend::Backend;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: DashMap::new(),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}