use crate::PreProcessedDocument;
use crate::preprocessor::RedirectReference;
use crate::utils::extract_plain_text;
use crate::wiki::{DocumentNamespace, check_documents_exist};
use anyhow::Result;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use sevenmark_parser::ast::{ResolvedMediaInfo, SevenMarkElement, Traversable};
use std::collections::{HashMap, HashSet};
use tracing::debug;
use utoipa::ToSchema;

/// Final result after media resolution
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProcessedDocument {
    pub categories: HashSet<String>,
    pub redirect: Option<RedirectReference>,
    pub references: HashSet<(DocumentNamespace, String)>,
    #[schema(value_type = Vec<Object>)]
    pub ast: Vec<SevenMarkElement>,
}

/// Processes document with media resolution
pub async fn postprocess_sevenmark(
    preprocessed: PreProcessedDocument,
    db: &DatabaseConnection,
) -> Result<ProcessedDocument> {
    let mut ast = preprocessed.ast;

    // If no media references, return immediately
    if preprocessed.media.is_empty() {
        return Ok(ProcessedDocument {
            categories: preprocessed.categories,
            redirect: preprocessed.redirect,
            references: preprocessed.references,
            ast,
        });
    }

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

    // Build resolution map from existence check results
    let mut resolved_map: HashMap<(DocumentNamespace, String), ResolvedMediaInfo> = HashMap::new();

    for result in existence_results {
        let key = (result.namespace.clone(), result.title.clone());

        let resolved = match result.namespace {
            DocumentNamespace::File => {
                // For files, use file_url if exists
                if let Some(file_url) = result.file_url {
                    ResolvedMediaInfo {
                        resolved_url: file_url,
                        is_valid: Some(true),
                    }
                } else {
                    ResolvedMediaInfo {
                        resolved_url: String::new(),
                        is_valid: Some(false),
                    }
                }
            }
            DocumentNamespace::Document => {
                // For documents, generate /document/{title} URL
                ResolvedMediaInfo {
                    resolved_url: format!("/document/{}", result.title),
                    is_valid: Some(result.exists),
                }
            }
            DocumentNamespace::Category => {
                // For categories, generate /category/{title} URL
                ResolvedMediaInfo {
                    resolved_url: format!("/category/{}", result.title),
                    is_valid: Some(result.exists),
                }
            }
        };

        resolved_map.insert(key, resolved);
    }

    // Traverse AST and resolve MediaElement references
    resolve_media_elements(&mut ast, &resolved_map);

    Ok(ProcessedDocument {
        categories: preprocessed.categories,
        redirect: preprocessed.redirect,
        references: preprocessed.references,
        ast,
    })
}

fn resolve_media_elements(
    elements: &mut [SevenMarkElement],
    resolved_map: &HashMap<(DocumentNamespace, String), ResolvedMediaInfo>,
) {
    for element in elements {
        resolve_media_recursive(element, resolved_map);
    }
}

fn resolve_media_recursive(
    element: &mut SevenMarkElement,
    resolved_map: &HashMap<(DocumentNamespace, String), ResolvedMediaInfo>,
) {
    if let SevenMarkElement::MediaElement(media) = element {
        // Priority: file > document > category > url

        // Check #file parameter
        if let Some(file_param) = media.parameters.get("file") {
            let title = extract_plain_text(&file_param.value);
            if !title.is_empty() {
                let key = (DocumentNamespace::File, title.clone());
                if let Some(resolved) = resolved_map.get(&key) {
                    media.resolved_info = Some(resolved.clone());
                    return;
                } else {
                    // File was in the request but not found in response
                    media.resolved_info = Some(ResolvedMediaInfo {
                        resolved_url: String::new(),
                        is_valid: Some(false),
                    });
                    return;
                }
            }
        }

        // Check #document parameter
        if let Some(doc_param) = media.parameters.get("document") {
            let title = extract_plain_text(&doc_param.value);
            if !title.is_empty() {
                let key = (DocumentNamespace::Document, title.clone());
                if let Some(resolved) = resolved_map.get(&key) {
                    media.resolved_info = Some(resolved.clone());
                    return;
                } else {
                    // Document not found
                    media.resolved_info = Some(ResolvedMediaInfo {
                        resolved_url: format!("/document/{}", title),
                        is_valid: Some(false),
                    });
                    return;
                }
            }
        }

        // Check #category parameter
        if let Some(cat_param) = media.parameters.get("category") {
            let title = extract_plain_text(&cat_param.value);
            if !title.is_empty() {
                let key = (DocumentNamespace::Category, title.clone());
                if let Some(resolved) = resolved_map.get(&key) {
                    media.resolved_info = Some(resolved.clone());
                    return;
                } else {
                    // Category not found
                    media.resolved_info = Some(ResolvedMediaInfo {
                        resolved_url: format!("/category/{}", title),
                        is_valid: Some(false),
                    });
                    return;
                }
            }
        }

        // Check #url parameter (external link - no is_valid needed)
        if let Some(url_param) = media.parameters.get("url") {
            let url = extract_plain_text(&url_param.value);
            if !url.is_empty() {
                media.resolved_info = Some(ResolvedMediaInfo {
                    resolved_url: url,
                    is_valid: None, // 외부 링크는 존재 여부 개념 없음
                });
                return;
            }
        }
    }

    // Traverse children
    element.traverse_children(&mut |child| {
        resolve_media_recursive(child, resolved_map);
    });
}
