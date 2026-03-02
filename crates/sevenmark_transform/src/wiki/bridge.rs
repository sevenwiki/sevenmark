use super::entity::{
    DocumentFiles, DocumentFilesColumn, DocumentMetadata, DocumentMetadataColumn,
    DocumentRevisions, DocumentRevisionsColumn,
};
use super::revision_storage::RevisionStorageClient;
use super::types::{DocumentExistence, DocumentNamespace, DocumentResponse, DocumentRevision};
use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::{HashMap, HashSet};
use tracing::{debug, warn};
use uuid::Uuid;

const MAX_REVISION_DOWNLOAD_CONCURRENCY: usize = 16;
const MAX_METADATA_QUERY_CONCURRENCY: usize = 4;
const MAX_TITLES_PER_METADATA_QUERY: usize = 1000;

fn group_requests_by_namespace(
    requests: &[(DocumentNamespace, String)],
) -> HashMap<DocumentNamespace, Vec<String>> {
    let mut grouped: HashMap<DocumentNamespace, HashSet<String>> = HashMap::new();

    for (namespace, title) in requests {
        grouped
            .entry(namespace.clone())
            .or_default()
            .insert(title.clone());
    }

    grouped
        .into_iter()
        .map(|(namespace, titles)| (namespace, titles.into_iter().collect()))
        .collect()
}

async fn fetch_metadata_batch(
    db: &DatabaseConnection,
    requests: &[(DocumentNamespace, String)],
) -> Result<Vec<super::entity::document_metadata::Model>> {
    let by_namespace = group_requests_by_namespace(requests);
    if by_namespace.is_empty() {
        return Ok(Vec::new());
    }

    let mut query_jobs = Vec::new();
    for (namespace, titles) in by_namespace {
        for chunk in titles.chunks(MAX_TITLES_PER_METADATA_QUERY) {
            query_jobs.push((namespace.clone(), chunk.to_vec()));
        }
    }

    let mut stream = stream::iter(query_jobs.into_iter().map(|(namespace, titles)| {
        let db = db.clone();
        async move {
            DocumentMetadata::find()
                .filter(DocumentMetadataColumn::Namespace.eq(namespace))
                .filter(DocumentMetadataColumn::Title.is_in(titles))
                .all(&db)
                .await
        }
    }))
    .buffer_unordered(MAX_METADATA_QUERY_CONCURRENCY);

    let mut metadata_list = Vec::new();
    while let Some(result) = stream.next().await {
        metadata_list.extend(result.context("Failed to fetch document metadata")?);
    }

    Ok(metadata_list)
}

/// Fetch multiple documents by namespace and title using Sea ORM
pub async fn fetch_documents_batch(
    db: &DatabaseConnection,
    revision_storage: &RevisionStorageClient,
    requests: Vec<(DocumentNamespace, String)>,
) -> Result<Vec<DocumentResponse>> {
    if requests.is_empty() {
        debug!("No documents to fetch");
        return Ok(Vec::new());
    }

    debug!("Fetching {} documents from database", requests.len());

    // Query document metadata
    let metadata_list = fetch_metadata_batch(db, &requests).await?;

    if metadata_list.is_empty() {
        debug!("No documents found");
        return Ok(Vec::new());
    }

    // Extract revision IDs
    let revision_ids: Vec<Uuid> = metadata_list
        .iter()
        .filter_map(|doc| doc.current_revision_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Fetch revisions in batch (get storage_keys from DB, content from SeaweedFS)
    let mut revisions_map: HashMap<Uuid, String> = HashMap::new();
    if !revision_ids.is_empty() {
        let revisions = DocumentRevisions::find()
            .filter(DocumentRevisionsColumn::Id.is_in(revision_ids))
            .all(db)
            .await
            .context("Failed to fetch document revisions")?;

        // Download revision blobs with bounded concurrency to avoid burst load.
        let mut content_stream = stream::iter(revisions.into_iter().map(|rev| {
            let revision_storage = revision_storage.clone();
            let storage_key = rev.storage_key;
            let revision_id = rev.id;
            async move {
                (
                    revision_id,
                    revision_storage.download_content(&storage_key).await,
                )
            }
        }))
        .buffer_unordered(MAX_REVISION_DOWNLOAD_CONCURRENCY);

        // Collect successful downloads
        while let Some((revision_id, result)) = content_stream.next().await {
            match result {
                Ok(content) => {
                    revisions_map.insert(revision_id, content);
                }
                Err(e) => {
                    warn!(revision_id = %revision_id, error = %e, "Failed to download content");
                    // Skip this revision if download fails
                }
            }
        }
    }

    // Extract File namespace document IDs
    let file_document_ids: Vec<Uuid> = metadata_list
        .iter()
        .filter(|doc| doc.namespace == DocumentNamespace::File)
        .map(|doc| doc.id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Fetch file storage keys in batch
    let mut storage_key_map: HashMap<Uuid, String> = HashMap::new();
    if !file_document_ids.is_empty() {
        let files = DocumentFiles::find()
            .filter(DocumentFilesColumn::DocumentId.is_in(file_document_ids))
            .all(db)
            .await
            .context("Failed to fetch document files")?;

        for file in files {
            storage_key_map.insert(file.document_id, file.storage_key);
        }
    }

    // Build response
    let mut documents = Vec::new();
    for doc in metadata_list {
        let current_revision_id = match doc.current_revision_id {
            Some(id) => id,
            None => continue, // Skip documents without current revision
        };

        let content = match revisions_map.get(&current_revision_id) {
            Some(content) => content.clone(),
            None => continue, // Skip if revision not found
        };

        documents.push(DocumentResponse {
            id: doc.id.to_string(),
            namespace: doc.namespace,
            title: doc.title,
            current_revision: DocumentRevision { content },
            file_url: storage_key_map.get(&doc.id).cloned(),
        });
    }

    debug!("Successfully fetched {} documents", documents.len());
    Ok(documents)
}

/// Check if documents exist without fetching content (lightweight)
/// Used for link coloring (red/blue links)
pub async fn check_documents_exist(
    db: &DatabaseConnection,
    requests: Vec<(DocumentNamespace, String)>,
) -> Result<Vec<DocumentExistence>> {
    if requests.is_empty() {
        debug!("No documents to check");
        return Ok(Vec::new());
    }

    debug!("Checking existence of {} documents", requests.len());

    // Query document metadata only (no revisions)
    let metadata_list = fetch_metadata_batch(db, &requests)
        .await
        .context("Failed to check document existence")?;

    // Build set of existing documents
    let mut existing: HashMap<(DocumentNamespace, String), Uuid> = HashMap::new();
    for doc in &metadata_list {
        existing.insert((doc.namespace.clone(), doc.title.clone()), doc.id);
    }

    // Fetch file URLs only for File namespace documents that exist
    let file_document_ids: Vec<Uuid> = metadata_list
        .iter()
        .filter(|doc| doc.namespace == DocumentNamespace::File)
        .map(|doc| doc.id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Store (storage_key, width, height) for file documents
    let mut file_info_map: HashMap<Uuid, (String, i32, i32)> = HashMap::new();
    if !file_document_ids.is_empty() {
        let files = DocumentFiles::find()
            .filter(DocumentFilesColumn::DocumentId.is_in(file_document_ids))
            .all(db)
            .await
            .context("Failed to fetch document files")?;

        for file in files {
            file_info_map.insert(
                file.document_id,
                (file.storage_key, file.width, file.height),
            );
        }
    }

    // Build response for all requested documents
    let results: Vec<DocumentExistence> = requests
        .into_iter()
        .map(|(namespace, title)| {
            let doc_id = existing.get(&(namespace.clone(), title.clone()));
            let (file_url, file_width, file_height) = if let Some(id) = doc_id {
                if let Some((url, w, h)) = file_info_map.get(id) {
                    (Some(url.clone()), Some(*w), Some(*h))
                } else {
                    (None, None, None)
                }
            } else {
                (None, None, None)
            };

            DocumentExistence {
                namespace,
                title,
                exists: doc_id.is_some(),
                file_url,
                file_width,
                file_height,
            }
        })
        .collect();

    debug!(
        "Checked {} documents, {} exist",
        results.len(),
        results.iter().filter(|r| r.exists).count()
    );
    Ok(results)
}
