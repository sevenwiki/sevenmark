use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub is_dev: bool,

    pub server_host: String,
    pub server_port: String,
}

// LazyLock으로 자동 초기화
static CONFIG: LazyLock<DbConfig> = LazyLock::new(|| {
    dotenv().ok();

    let is_dev = matches!(
        env::var("ENVIRONMENT").as_deref(),
        Ok("dev") | Ok("development")
    );

    DbConfig {
        is_dev,

        server_host: env::var("HOST").expect("HOST must be set in .env file"),
        server_port: env::var("PORT").expect("PORT must be set in .env file"),
    }
});

impl DbConfig {
    pub fn get() -> &'static DbConfig {
        &CONFIG
    }
}
