use crate::sevenmark::transform::WikiClient;

#[derive(Clone)]
pub struct AppState {
    pub wiki_client: WikiClient
}
