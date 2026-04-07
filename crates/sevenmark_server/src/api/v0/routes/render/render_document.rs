use super::sort_strings;
use crate::errors::errors::Errors;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sevenmark_html::{RenderConfig, render_document_with_spans};
use sevenmark_parser::core::parse_document;
use sevenmark_transform::preprocessor::{DocumentReference, RedirectReference, SectionInfo};
use sevenmark_transform::process_sevenmark;
use sevenmark_transform::wiki::DocumentNamespace;
use std::collections::HashSet;
use tokio::task::spawn_blocking;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RenderDocumentRequest {
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
pub struct RenderedDocument {
    /// Rendered HTML content
    pub html: String,
    /// Document categories
    pub categories: Vec<String>,
    /// Redirect target if this is a redirect page
    #[schema(value_type = Option<Object>)]
    pub redirect: Option<RedirectReference>,
    /// Referenced documents (for backlinks)
    #[schema(value_type = Vec<Object>)]
    pub references: Vec<DocumentReference>,
    /// User mention UUIDs collected from the document
    pub user_mentions: Vec<String>,
    /// Section information with byte offsets for section editing
    pub sections: Vec<SectionInfo>,
}

fn namespace_sort_key(namespace: &DocumentNamespace) -> u8 {
    match namespace {
        DocumentNamespace::Document => 0,
        DocumentNamespace::File => 1,
        DocumentNamespace::Category => 2,
        DocumentNamespace::User => 3,
    }
}

fn sort_references(references: HashSet<DocumentReference>) -> Vec<DocumentReference> {
    let mut references: Vec<_> = references.into_iter().collect();
    references.sort_by(|a, b| {
        namespace_sort_key(&a.namespace)
            .cmp(&namespace_sort_key(&b.namespace))
            .then_with(|| a.title.cmp(&b.title))
    });
    references
}

#[utoipa::path(
    post,
    path = "/v0/render-document",
    request_body = RenderDocumentRequest,
    responses(
        (status = 200, description = "Document rendered successfully", body = RenderedDocument),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Render"
)]
pub async fn render_document(
    State(state): State<AppState>,
    Json(payload): Json<RenderDocumentRequest>,
) -> Result<Json<RenderedDocument>, Errors> {
    let RenderDocumentRequest {
        content,
        file_base_url,
        document_base_url,
        category_base_url,
        user_base_url,
    } = payload;

    // Parse
    let (content, ast) = spawn_blocking(move || {
        let ast = parse_document(&content);
        (content, ast)
    })
    .await
    .map_err(|e| Errors::SysInternalError(format!("Parser task failed: {e}")))?;

    // Process (resolve includes, media, etc.)
    let processed = process_sevenmark(ast, &state.conn, &state.revision_storage)
        .await
        .map_err(|e| Errors::SysInternalError(e.to_string()))?;

    // Render to HTML with span data attributes for editor sync
    let config = RenderConfig {
        file_base_url: Some(&file_base_url),
        document_base_url: Some(&document_base_url),
        category_base_url: Some(&category_base_url),
        user_base_url: Some(&user_base_url),
    };
    let html = render_document_with_spans(&processed.ast, &config, &content);

    Ok(Json(RenderedDocument {
        html,
        categories: sort_strings(processed.categories),
        redirect: processed.redirect,
        references: sort_references(processed.references),
        user_mentions: sort_strings(processed.user_mentions),
        sections: processed.sections,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_strings_returns_stable_ascending_order() {
        let values = HashSet::from(["gamma".to_string(), "alpha".to_string(), "beta".to_string()]);

        assert_eq!(
            sort_strings(values),
            vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
        );
    }

    #[test]
    fn sort_references_orders_by_namespace_then_title() {
        let references = HashSet::from([
            DocumentReference {
                namespace: DocumentNamespace::User,
                title: "charlie".to_string(),
            },
            DocumentReference {
                namespace: DocumentNamespace::Document,
                title: "bravo".to_string(),
            },
            DocumentReference {
                namespace: DocumentNamespace::Document,
                title: "alpha".to_string(),
            },
            DocumentReference {
                namespace: DocumentNamespace::Category,
                title: "delta".to_string(),
            },
        ]);

        let sorted = sort_references(references);
        let actual: Vec<_> = sorted
            .into_iter()
            .map(|r| (namespace_sort_key(&r.namespace), r.title))
            .collect();

        assert_eq!(
            actual,
            vec![
                (0, "alpha".to_string()),
                (0, "bravo".to_string()),
                (2, "delta".to_string()),
                (3, "charlie".to_string()),
            ]
        );
    }
}
