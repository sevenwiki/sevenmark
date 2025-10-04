use axum::Router;
use axum::routing::post;
use super::parse::parse_endpoint;
use crate::state::AppState;

pub fn parse_routes(state: AppState) -> Router<AppState> {
    Router::new().route("/parse", post(parse_endpoint))
}