use super::health::routes::health_routes as HealthRoutes;
use super::openapi::ApiDoc;
use super::parse::routes::parse_routes as ParseRoutes;
use crate::errors::errors::handler_404;
use crate::state::AppState;
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

/// API + Swagger UI 라우터 통합
pub fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::merged()))
        .nest("/v0", HealthRoutes())
        .nest("/v0", ParseRoutes(state.clone()))
        .fallback(handler_404)
}
