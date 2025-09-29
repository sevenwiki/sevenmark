use super::health::openapi::HealthApiDoc;

use crate::errors::errors::ErrorResponse;
use utoipa::openapi::security::{ApiKey, ApiKeyValue};
use utoipa::{Modify, OpenApi, openapi::security::SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    components(
        schemas(
            ErrorResponse,
        )
    ),
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn merged() -> utoipa::openapi::OpenApi {
        let mut openapi = Self::openapi();
        openapi.merge(HealthApiDoc::openapi());
        openapi
    }
}