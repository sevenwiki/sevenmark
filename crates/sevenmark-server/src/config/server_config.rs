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

static CONFIG: LazyLock<ServerConfig> = LazyLock::new(|| {
    dotenv().ok();

    let mut errors: Vec<String> = Vec::new();

    macro_rules! require {
        ($name:expr) => {
            env::var($name).unwrap_or_else(|_| {
                errors.push(format!("  - {} (missing)", $name));
                String::new()
            })
        };
    }

    let db_user = require!("POSTGRES_USER");
    let db_password = require!("POSTGRES_PASSWORD");
    let db_host = require!("POSTGRES_HOST");
    let db_port = require!("POSTGRES_PORT");
    let db_name = require!("POSTGRES_NAME");
    let server_host = require!("HOST");
    let server_port = require!("PORT");
    let r2_endpoint = require!("R2_ENDPOINT");
    let r2_access_key_id = require!("R2_ACCESS_KEY_ID");
    let r2_secret_access_key = require!("R2_SECRET_ACCESS_KEY");
    let r2_assets_bucket_name = require!("R2_ASSETS_BUCKET_NAME");
    let r2_assets_public_domain = require!("R2_ASSETS_PUBLIC_DOMAIN");
    let r2_revision_bucket_name = require!("R2_REVISION_BUCKET_NAME");

    if !errors.is_empty() {
        panic!(
            "\n\nMissing or invalid environment variables ({} errors):\n{}\n",
            errors.len(),
            errors.join("\n")
        );
    }

    ServerConfig {
        db_user,
        db_password,
        db_host,
        db_port,
        db_name,
        db_max_connection: env::var("POSTGRES_MAX_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        db_min_connection: env::var("POSTGRES_MIN_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),

        server_host,
        server_port,

        // Cloudflare R2
        r2_endpoint,
        r2_region: env::var("R2_REGION").unwrap_or_else(|_| "auto".into()),
        r2_access_key_id,
        r2_secret_access_key,
        r2_assets_bucket_name,
        r2_assets_public_domain,
        r2_revision_bucket_name,
    }
});

impl ServerConfig {
    pub fn get() -> &'static ServerConfig {
        &CONFIG
    }
}
