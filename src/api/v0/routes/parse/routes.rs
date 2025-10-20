use super::parse::parse_endpoint;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub fn parse_routes(state: AppState) -> Router<AppState> {
    Router::new().route("/parse", post(parse_endpoint))
}
