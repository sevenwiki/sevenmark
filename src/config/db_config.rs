use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub is_dev: bool,

    pub wiki_server_host: String,
    pub wiki_server_port: String,

    pub server_host: String,
    pub server_port: String,
}

// LazyLock
static CONFIG: LazyLock<DbConfig> = LazyLock::new(|| {
    dotenv().ok();

    let is_dev = matches!(
        env::var("ENVIRONMENT").as_deref(),
        Ok("dev") | Ok("development")
    );

    DbConfig {
        is_dev,

        wiki_server_host: env::var("WIKI_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        wiki_server_port: env::var("WIKI_SERVER_PORT").unwrap_or_else(|_| "8000".to_string()),

        server_host: env::var("HOST").expect("HOST must be set in .env file"),
        server_port: env::var("PORT").expect("PORT must be set in .env file"),
    }
});

impl DbConfig {
    pub fn get() -> &'static DbConfig {
        &CONFIG
    }
}
