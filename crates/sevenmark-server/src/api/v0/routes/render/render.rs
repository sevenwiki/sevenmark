use crate::errors::errors::Errors;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sevenmark_html::render_document;
use sevenmark_parser::core::parse_document;
use sevenmark_transform::preprocessor::{DocumentReference, RedirectReference, SectionInfo};
use sevenmark_transform::process_sevenmark;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RenderDocumentRequest {
    /// Raw SevenMark content to render
    pub content: String,
    /// Edit URL prefix for section edit links (e.g., "/edit/DocumentTitle")
    pub edit_url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RenderedDocument {
    /// Rendered HTML content
    pub html: String,
    /// Document categories
    pub categories: HashSet<String>,
    /// Redirect target if this is a redirect page
    #[schema(value_type = Option<Object>)]
    pub redirect: Option<RedirectReference>,
    /// Referenced documents (for backlinks)
    #[schema(value_type = Vec<Object>)]
    pub references: HashSet<DocumentReference>,
    /// Section information with byte offsets for section editing
    pub sections: Vec<SectionInfo>,
}

#[utoipa::path(
    post,
    path = "/v0/render",
    request_body = RenderDocumentRequest,
    responses(
        (status = 200, description = "Document rendered successfully", body = RenderedDocument),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Render"
)]
pub async fn render_endpoint(
    State(state): State<AppState>,
    Json(payload): Json<RenderDocumentRequest>,
) -> Result<Json<RenderedDocument>, Errors> {
    // Parse
    let ast = parse_document(payload.content.as_str());

    // Process (resolve includes, media, etc.)
    let processed = process_sevenmark(ast, &state.conn)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;

    // Render to HTML
    let html = render_document(&processed.ast, &payload.edit_url);

    Ok(Json(RenderedDocument {
        html,
        categories: processed.categories,
        redirect: processed.redirect,
        references: processed.references,
        sections: processed.sections,
    }))
}