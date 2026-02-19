mod backend;
mod diagnostics;
mod document;
mod folding;

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