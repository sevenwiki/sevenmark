use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_max_connection: u32,
    pub db_min_connection: u32,

    pub server_host: String,
    pub server_port: String,

    // Cloudflare R2 (shared credentials)
    pub r2_endpoint: String,
    pub r2_region: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    // R2 Assets (public bucket - images, sitemap)
    pub r2_assets_bucket_name: String,
    pub r2_assets_public_domain: String,
    // R2 Revision (private bucket - revision content)
    pub r2_revision_bucket_name: String,
}

// LazyLock
static CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| {
    dotenv().ok();

    ServerConfig {
        db_user: env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        db_password: env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        db_host: env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set"),
        db_port: env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set"),
        db_name: env::var("POSTGRES_NAME").expect("POSTGRES_NAME must be set"),
        db_max_connection: env::var("POSTGRES_MAX_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        db_min_connection: env::var("POSTGRES_MIN_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),

        server_host: env::var("HOST").expect("HOST must be set in .env file"),
        server_port: env::var("PORT").expect("PORT must be set in .env file"),

        // Cloudflare R2
        r2_endpoint: env::var("R2_ENDPOINT").expect("R2_ENDPOINT must be set"),
        r2_region: env::var("R2_REGION").unwrap_or_else(|_| "auto".into()),
        r2_access_key_id: env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set"),
        r2_secret_access_key: env::var("R2_SECRET_ACCESS_KEY")
            .expect("R2_SECRET_ACCESS_KEY must be set"),
        r2_assets_bucket_name: env::var("R2_ASSETS_BUCKET_NAME")
            .expect("R2_ASSETS_BUCKET_NAME must be set"),
        r2_assets_public_domain: env::var("R2_ASSETS_PUBLIC_DOMAIN")
            .expect("R2_ASSETS_PUBLIC_DOMAIN must be set"),
        r2_revision_bucket_name: env::var("R2_REVISION_BUCKET_NAME")
            .expect("R2_REVISION_BUCKET_NAME must be set"),
    }
});

impl ServerConfig {
    pub fn get() -> &'static ServerConfig {
        &CONFIG
    }
}
