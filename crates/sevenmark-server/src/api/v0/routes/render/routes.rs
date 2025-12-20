use super::render::render_endpoint;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub fn render_routes(_state: AppState) -> Router<AppState> {
    Router::new().route("/render", post(render_endpoint))
}
