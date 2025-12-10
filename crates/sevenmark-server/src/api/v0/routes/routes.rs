use super::parse::routes::parse_routes as ParseRoutes;
use crate::state::AppState;
use axum::Router;

/// v0 API 라우터
pub fn v0_routes(state: AppState) -> Router<AppState> {
    Router::new().merge(ParseRoutes(state))
}
