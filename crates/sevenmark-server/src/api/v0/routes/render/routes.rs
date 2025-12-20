use super::render::render_endpoint;
use crate::state::AppState;
use axum::routing::post;
use axum::Router;

pub fn render_routes(_state: AppState) -> Router<AppState> {
    Router::new().route("/render", post(render_endpoint))
}