use crate::errors::errors::Errors;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sevenmark_html::{RenderConfig, render_document as render_html};
use sevenmark_parser::core::parse_document;
use sevenmark_transform::process_sevenmark;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RenderDiscussionRequest {
    /// Raw SevenMark content to render
    pub content: String,
    /// Base URL for file/media (e.g., Cloudflare CDN URL)
    pub file_base_url: String,
    /// Base URL for document links (e.g., "/Document/")
    pub document_base_url: String,
    /// Base URL for category links (e.g., "/Category/")
    pub category_base_url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RenderedDiscussion {
    /// Rendered HTML content
    pub html: String,
    /// User mention UUIDs collected from the content
    pub user_mentions: HashSet<String>,
}

#[utoipa::path(
    post,
    path = "/v0/render-discussion",
    request_body = RenderDiscussionRequest,
    responses(
        (status = 200, description = "Discussion rendered successfully", body = RenderedDiscussion),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Render"
)]
pub async fn render_discussion(
    State(state): State<AppState>,
    Json(payload): Json<RenderDiscussionRequest>,
) -> Result<Json<RenderedDiscussion>, Errors> {
    // Parse
    let ast = parse_document(payload.content.as_str());

    // Process (resolve includes, media, etc.)
    let processed = process_sevenmark(ast, &state.conn, &state.seaweedfs)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;

    // Render to HTML (no edit links for discussions)
    let config = RenderConfig {
        edit_url: None,
        file_base_url: Some(&payload.file_base_url),
        document_base_url: Some(&payload.document_base_url),
        category_base_url: Some(&payload.category_base_url),
    };
    let html = render_html(&processed.ast, &config);

    Ok(Json(RenderedDiscussion {
        html,
        user_mentions: processed.user_mentions,
    }))
}
