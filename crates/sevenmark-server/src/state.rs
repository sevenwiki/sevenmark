use sea_orm::DatabaseConnection as PostgresqlClient;
use sevenmark_transform::wiki::SeaweedFsClient;

#[derive(Clone)]
pub struct AppState {
    pub conn: PostgresqlClient,
    pub seaweedfs: SeaweedFsClient,
}
