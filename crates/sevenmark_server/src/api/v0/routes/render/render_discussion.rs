use super::sort_strings;
use crate::errors::errors::Errors;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sevenmark_html::{RenderConfig, render_document as render_html};
use sevenmark_parser::core::parse_document;
use sevenmark_transform::process_sevenmark;
use tokio::task::spawn_blocking;
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
    /// Base URL for user document links (e.g., "/User/")
    pub user_base_url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RenderedDiscussion {
    /// Rendered HTML content
    pub html: String,
    /// User mention UUIDs collected from the content
    pub user_mentions: Vec<String>,
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
    let RenderDiscussionRequest {
        content,
        file_base_url,
        document_base_url,
        category_base_url,
        user_base_url,
    } = payload;

    // Parse
    let ast = spawn_blocking(move || parse_document(&content))
        .await
        .map_err(|e| Errors::SysInternalError(format!("Parser task failed: {e}")))?;

    // Process (resolve includes, media, etc.)
    let processed = process_sevenmark(ast, &state.conn, &state.revision_storage)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;

    // Render to HTML (no edit links for discussions)
    let config = RenderConfig {
        edit_url: None,
        file_base_url: Some(&file_base_url),
        document_base_url: Some(&document_base_url),
        category_base_url: Some(&category_base_url),
        user_base_url: Some(&user_base_url),
    };
    let html = render_html(&processed.ast, &config);

    Ok(Json(RenderedDiscussion {
        html,
        user_mentions: sort_strings(processed.user_mentions),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn sort_strings_returns_stable_ascending_order() {
        let values = HashSet::from([
            "user-c".to_string(),
            "user-a".to_string(),
            "user-b".to_string(),
        ]);

        assert_eq!(
            sort_strings(values),
            vec![
                "user-a".to_string(),
                "user-b".to_string(),
                "user-c".to_string()
            ]
        );
    }
}
