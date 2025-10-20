use super::types::{DocumentNamespace, DocumentResponse, DocumentRevision};
use crate::config::db_config::DbConfig;
use anyhow::{Context, Result};
use sea_orm::{
    entity::prelude::*, ConnectOptions, Database, DatabaseConnection, Condition,
    ColumnTrait, EntityTrait, QueryFilter, DeriveActiveEnum,
};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info};

/// Establish database connection
pub async fn establish_connection() -> DatabaseConnection {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        &DbConfig::get().db_user,
        &DbConfig::get().db_password,
        &DbConfig::get().db_host,
        &DbConfig::get().db_port,
        &DbConfig::get().db_name
    );
    info!("Attempting to connect to database: {}", database_url);

    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(DbConfig::get().db_max_connection)
        .min_connections(DbConfig::get().db_min_connection)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false);

    match Database::connect(options).await {
        Ok(connection) => {
            info!("Successfully connected to the database.");
            connection
        }
        Err(err) => {
            error!("Failed to connect to database: {}", err);
            panic!("Failed to connect to database");
        }
    }
}

// ===== Minimal Entity Definitions =====

/// Document Namespace enum (matches DB enum)
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "document_namespace")]
pub enum DbDocumentNamespace {
    #[sea_orm(string_value = "document")]
    Document,
    #[sea_orm(string_value = "file")]
    File,
    #[sea_orm(string_value = "category")]
    Category,
}

/// Document Metadata Entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "document_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub namespace: DbDocumentNamespace,
    pub title: String,
    pub current_revision_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Document Revisions Entity
pub mod document_revisions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "document_revisions")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        #[sea_orm(column_type = "Text")]
        pub content: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Document Files Entity
pub mod document_files {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "document_files")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub document_id: Uuid,
        #[sea_orm(column_type = "Text")]
        pub storage_key: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// ===== Helper Functions =====

/// Convert DocumentNamespace to DbDocumentNamespace
fn to_db_namespace(namespace: &DocumentNamespace) -> DbDocumentNamespace {
    match namespace {
        DocumentNamespace::Document => DbDocumentNamespace::Document,
        DocumentNamespace::File => DbDocumentNamespace::File,
        DocumentNamespace::Category => DbDocumentNamespace::Category,
    }
}

/// Convert DbDocumentNamespace to DocumentNamespace
fn from_db_namespace(db_namespace: &DbDocumentNamespace) -> DocumentNamespace {
    match db_namespace {
        DbDocumentNamespace::Document => DocumentNamespace::Document,
        DbDocumentNamespace::File => DocumentNamespace::File,
        DbDocumentNamespace::Category => DocumentNamespace::Category,
    }
}

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
        let db_namespace = to_db_namespace(namespace);

        condition = condition.add(
            Condition::all()
                .add(Column::Namespace.eq(db_namespace))
                .add(Column::Title.eq(title.clone())),
        );
    }

    // Query document metadata
    let metadata_list = Entity::find()
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
        let revisions = document_revisions::Entity::find()
            .filter(document_revisions::Column::Id.is_in(revision_ids))
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
        .filter(|doc| doc.namespace == DbDocumentNamespace::File)
        .map(|doc| doc.id)
        .collect();

    // Fetch file storage keys in batch
    let mut storage_key_map: HashMap<Uuid, String> = HashMap::new();
    if !file_document_ids.is_empty() {
        let files = document_files::Entity::find()
            .filter(document_files::Column::DocumentId.is_in(file_document_ids))
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
            namespace: from_db_namespace(&doc.namespace),
            title: doc.title,
            current_revision: DocumentRevision { content },
            file_url: storage_key_map.get(&doc.id).cloned(),
        });
    }

    debug!("Successfully fetched {} documents", documents.len());
    Ok(documents)
}