use sea_orm::DatabaseConnection as PostgresqlClient;

#[derive(Clone)]
pub struct AppState {
    pub conn: PostgresqlClient,
}
