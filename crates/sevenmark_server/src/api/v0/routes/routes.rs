use super::render::routes::render_routes as RenderRoutes;
use crate::state::AppState;
use axum::Router;

/// v0 API 라우터
pub fn v0_routes(state: AppState) -> Router<AppState> {
    Router::new().merge(RenderRoutes(state))
}
