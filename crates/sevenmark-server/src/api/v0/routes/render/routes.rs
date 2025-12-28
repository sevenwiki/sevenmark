use super::render_discussion::render_discussion;
use super::render_document::render_document;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub fn render_routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/render-document", post(render_document))
        .route("/render-discussion", post(render_discussion))
}
