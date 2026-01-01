use crate::config::server_config::ServerConfig;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::RequestChecksumCalculation;
use sevenmark_transform::wiki::SeaweedFsClient;
use tracing::info;

/// 버킷 이름 (V7과 동일)
const BUCKET_NAME: &str = "v7-content";

pub async fn establish_seaweedfs_connection()
-> Result<SeaweedFsClient, Box<dyn std::error::Error + Send + Sync>> {
    let config = ServerConfig::get();

    info!("Connecting to SeaweedFS at: {}", config.seaweedfs_endpoint);

    // SeaweedFS S3 API - 내부 네트워크, 인증 없음
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .endpoint_url(&config.seaweedfs_endpoint)
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            "",
            "",
            None,
            None,
            "anonymous",
        ))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&aws_config)
        .force_path_style(true)
        .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
        .build();

    let client = Client::from_conf(s3_config);
    let seaweedfs_client = SeaweedFsClient::new(client, BUCKET_NAME.to_string());

    info!("Successfully connected to SeaweedFS");
    Ok(seaweedfs_client)
}
