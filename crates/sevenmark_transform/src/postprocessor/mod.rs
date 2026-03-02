mod resolver;
#[cfg(test)]
mod tests;

use crate::PreProcessedDocument;
use crate::preprocessor::{DocumentReference, RedirectReference, SectionInfo};
use crate::wiki::{DocumentNamespace, check_documents_exist};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use sevenmark_ast::Element;
use std::collections::{HashMap, HashSet};
use tracing::debug;
use utoipa::ToSchema;

use resolver::resolve_media_elements;

/// Media resolution map: (namespace, title) -> (file_url, width, height, is_valid)
pub(super) type MediaResolutionMap =
    HashMap<(DocumentNamespace, String), (Option<String>, Option<i32>, Option<i32>, bool)>;

/// Final result after media resolution
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProcessedDocument {
    pub categories: HashSet<String>,
    pub redirect: Option<RedirectReference>,
    pub references: HashSet<DocumentReference>,
    /// User mention UUIDs collected from the document
    pub user_mentions: HashSet<String>,
    #[schema(value_type = Vec<Object>)]
    pub ast: Vec<Element>,
    pub sections: Vec<SectionInfo>,
}

/// Processes document with media resolution
pub async fn postprocess_sevenmark(
    preprocessed: PreProcessedDocument,
    db: &DatabaseConnection,
) -> Result<ProcessedDocument> {
    let mut ast = preprocessed.ast;

    // Build resolution map from DB (only if there are media references)
    let resolved_map: MediaResolutionMap = if preprocessed.media.is_empty() {
        HashMap::new()
    } else {
        // Convert MediaReference to (namespace, title) tuples for batch request
        let requests: Vec<_> = preprocessed
            .media
            .into_iter()
            .map(|m| (m.namespace, m.title))
            .collect();

        debug!(
            "Checking existence of {} unique media references",
            requests.len()
        );

        // Check document existence (lightweight - no content fetching)
        let existence_results = check_documents_exist(db, requests).await?;

        let mut map = HashMap::new();

        for result in existence_results {
            let key = (result.namespace.clone(), result.title.clone());
            let value = match result.namespace {
                DocumentNamespace::File => {
                    // File: store file_url, width, height and validity
                    let is_valid = result.file_url.is_some();
                    (
                        result.file_url,
                        result.file_width,
                        result.file_height,
                        is_valid,
                    )
                }
                DocumentNamespace::Document
                | DocumentNamespace::Category
                | DocumentNamespace::User => {
                    // Document/Category/User: just store validity (title is in key)
                    (None, None, None, result.exists)
                }
            };
            map.insert(key, value);
        }

        map
    };

    // Always traverse AST to resolve MediaElement references (including #url)
    resolve_media_elements(&mut ast, &resolved_map);

    Ok(ProcessedDocument {
        categories: preprocessed.categories,
        redirect: preprocessed.redirect,
        references: preprocessed.references,
        user_mentions: preprocessed.user_mentions,
        ast,
        sections: preprocessed.sections,
    })
}
