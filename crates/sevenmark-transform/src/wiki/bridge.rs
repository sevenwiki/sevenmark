use super::entity::{
    DocumentFiles, DocumentFilesColumn, DocumentMetadata, DocumentMetadataColumn,
    DocumentRevisions, DocumentRevisionsColumn,
};
use super::types::{DocumentNamespace, DocumentResponse, DocumentRevision};
use anyhow::{Context, Result};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

/// Fetch multiple documents by namespace and title using Sea ORM
pub async fn fetch_documents_batch(
    db: &DatabaseConnection,
    requests: Vec<(DocumentNamespace, String)>,
) -> Result<Vec<DocumentResponse>> {
    if requests.is_empty() {
        debug!("No documents to fetch");
        return Ok(Vec::new());
    }

    debug!("Fetching {} documents from database", requests.len());

    // Build OR conditions for batch query
    let mut condition = Condition::any();
    for (namespace, title) in &requests {
        condition = condition.add(
            Condition::all()
                .add(DocumentMetadataColumn::Namespace.eq(namespace.clone()))
                .add(DocumentMetadataColumn::Title.eq(title.clone())),
        );
    }

    // Query document metadata
    let metadata_list = DocumentMetadata::find()
        .filter(condition)
        .all(db)
        .await
        .context("Failed to fetch document metadata")?;

    if metadata_list.is_empty() {
        debug!("No documents found");
        return Ok(Vec::new());
    }

    // Extract revision IDs
    let revision_ids: Vec<Uuid> = metadata_list
        .iter()
        .filter_map(|doc| doc.current_revision_id)
        .collect();

    // Fetch revisions in batch
    let mut revisions_map: HashMap<Uuid, String> = HashMap::new();
    if !revision_ids.is_empty() {
        let revisions = DocumentRevisions::find()
            .filter(DocumentRevisionsColumn::Id.is_in(revision_ids))
            .all(db)
            .await
            .context("Failed to fetch document revisions")?;

        for revision in revisions {
            revisions_map.insert(revision.id, revision.content);
        }
    }

    // Extract File namespace document IDs
    let file_document_ids: Vec<Uuid> = metadata_list
        .iter()
        .filter(|doc| doc.namespace == DocumentNamespace::File)
        .map(|doc| doc.id)
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
