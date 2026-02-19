use super::health_check::health_check;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn health_routes() -> Router<AppState> {
    Router::new().route("/health-check", get(health_check))
}
