mod define_if;
mod metadata;
mod references;
#[cfg(test)]
mod tests;

pub(super) use crate::text_utils::normalized_plain_text;
use crate::wiki::{DocumentNamespace, RevisionStorageClient, fetch_documents_batch};
use anyhow::Result;
use rayon::prelude::*;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use sevenmark_ast::Element;
use sevenmark_parser::core::parse_document;
use std::collections::{HashMap, HashSet};
use tracing::debug;

use define_if::process_defines_and_ifs;
use metadata::collect_metadata;
use references::{collect_includes, collect_references, substitute_includes};

/// Media reference with namespace and title
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct MediaReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Section range information for frontend consumption
#[derive(utoipa::ToSchema, Debug, Clone, Serialize)]
pub struct SectionInfo {
    /// Section index (same as Header's section_index)
    pub section_index: usize,
    /// Header level (1-6)
    pub level: usize,
    /// Section start byte offset (header start position)
    pub start: usize,
    /// Section end byte offset (next same/higher level header or document end)
    pub end: usize,
}

/// Redirect reference with namespace and title
#[derive(utoipa::ToSchema, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct RedirectReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Document reference with namespace and title
#[derive(utoipa::ToSchema, Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct DocumentReference {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// Final result after include resolution
#[derive(Debug, Clone, Serialize)]
pub struct PreProcessedDocument {
    pub media: HashSet<MediaReference>,
    pub categories: HashSet<String>,
    pub redirect: Option<RedirectReference>,
    pub references: HashSet<DocumentReference>,
    /// User mention UUIDs collected from the document
    pub user_mentions: HashSet<String>,
    pub ast: Vec<Element>,
    pub sections: Vec<SectionInfo>,
}

pub(super) const DEFAULT_NAMESPACE: &str = "Document";

pub(super) fn parse_namespace(namespace: &str) -> DocumentNamespace {
    match namespace {
        "Document" => DocumentNamespace::Document,
        "File" => DocumentNamespace::File,
        "User" => DocumentNamespace::User,
        "Category" => DocumentNamespace::Category,
        _ => DocumentNamespace::Document,
    }
}

/// Processes document with 1-depth include resolution
pub async fn preprocess_sevenmark(
    mut ast: Vec<Element>,
    db: &DatabaseConnection,
    revision_storage: &RevisionStorageClient,
) -> Result<PreProcessedDocument> {
    // Process defines and ifs in document order (single pass)
    let mut variables = HashMap::new();
    process_defines_and_ifs(&mut ast, &mut variables);

    // Collect metadata from main document
    let mut categories = HashSet::new();
    let mut redirect = None;
    let mut all_media = HashSet::new();
    let mut sections = Vec::new();
    let mut user_mentions = HashSet::new();

    collect_metadata(
        &ast,
        &mut categories,
        &mut redirect,
        &mut all_media,
        &mut sections,
        &mut user_mentions,
        true,
    );

    // Collect unique includes for fetching (only Include elements need content fetching)
    let mut includes_to_fetch = HashSet::new();
    collect_includes(&ast, &mut includes_to_fetch);

    if !includes_to_fetch.is_empty() {
        // Prepare batch fetch requests
        let requests: Vec<_> = includes_to_fetch
            .iter()
            .map(|r| (r.namespace.clone(), r.title.clone()))
            .collect();

        debug!("Fetching {} unique documents", requests.len());

        // Fetch all documents
        let fetched_docs = fetch_documents_batch(db, revision_storage, requests).await?;

        // Parse fetched documents and store in map
        let docs_map: HashMap<DocumentReference, Vec<Element>> = fetched_docs
            .into_par_iter()
            .map(|doc| {
                let parsed_ast = parse_document(&doc.current_revision.content);
                (
                    DocumentReference {
                        namespace: doc.namespace,
                        title: doc.title,
                    },
                    parsed_ast,
                )
            })
            .collect();

        // Substitute includes with their content
        substitute_includes(&mut ast, &docs_map, &mut all_media);
    }

    // Collect all references from final AST
    let mut all_references = includes_to_fetch;
    collect_references(&ast, &mut all_references);

    Ok(PreProcessedDocument {
        ast,
        media: all_media,
        categories,
        redirect,
        references: all_references,
        user_mentions,
        sections,
    })
}
