use crate::errors::errors::Errors;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sevenmark_html::{DISCUSSION_CONFIG, render_document as render_html};
use sevenmark_parser::core::parse_document;
use sevenmark_transform::process_sevenmark;
use std::collections::HashSet;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RenderDiscussionRequest {
    /// Raw SevenMark content to render
    pub content: String,
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
    let processed = process_sevenmark(ast, &state.conn)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;

    // Render to HTML (no edit links for discussions)
    let html = render_html(&processed.ast, &DISCUSSION_CONFIG);

    Ok(Json(RenderedDiscussion {
        html,
        user_mentions: processed.user_mentions,
    }))
}
