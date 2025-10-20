use crate::errors::errors::Errors;
use crate::parse_document;
use crate::sevenmark::transform::{ProcessedDocument, process_sevenmark};
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ParseDocumentRequest {
    pub content: String,
}

#[utoipa::path(
    post,
    path = "/v0/parse",
    request_body = ParseDocumentRequest,
    responses(
        (status = 200, description = "Document parsed successfully", body = ProcessedDocument),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Parse"
)]
pub async fn parse_endpoint(
    State(state): State<AppState>,
    Json(payload): Json<ParseDocumentRequest>,
) -> Result<Json<ProcessedDocument>, Errors> {
    let ast = parse_document(payload.content.as_str());
    let result = process_sevenmark(ast, &state.conn)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;
    Ok(Json(result))
}
