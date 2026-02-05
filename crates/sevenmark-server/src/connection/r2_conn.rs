use crate::config::server_config::ServerConfig;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use sevenmark_transform::wiki::RevisionStorageClient;
use tracing::info;

pub async fn establish_revision_storage_connection()
-> Result<RevisionStorageClient, Box<dyn std::error::Error + Send + Sync>> {
    let config = ServerConfig::get();

    info!(
        "Connecting to R2 revision storage at: {} (region: {})",
        config.r2_endpoint, config.r2_region
    );

    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.r2_region.clone()))
        .endpoint_url(&config.r2_endpoint)
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            &config.r2_access_key_id,
            &config.r2_secret_access_key,
            None,
            None,
            "r2-credentials",
        ))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&aws_config)
        .force_path_style(true)
        .build();

    let client = Client::from_conf(s3_config);
    let revision_storage =
        RevisionStorageClient::new(client, config.r2_revision_bucket_name.clone());

    info!("Successfully connected to R2 revision storage");
    Ok(revision_storage)
}
