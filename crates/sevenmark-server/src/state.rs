use sea_orm::DatabaseConnection as PostgresqlClient;
use sevenmark_transform::wiki::RevisionStorageClient;

#[derive(Clone)]
pub struct AppState {
    pub conn: PostgresqlClient,
    pub revision_storage: RevisionStorageClient,
}
