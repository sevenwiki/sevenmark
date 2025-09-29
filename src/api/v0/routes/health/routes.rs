use super::health_check::health_check;
use crate::state::AppState;
use axum::{routing::get, Router};


pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health_check", get(health_check))
}