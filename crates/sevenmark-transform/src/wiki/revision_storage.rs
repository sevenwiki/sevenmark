//! R2 revision storage client (S3 compatible)

use aws_sdk_s3::Client;
use std::sync::Arc;

/// Revision storage client (Cloudflare R2, S3 compatible API)
#[derive(Clone)]
pub struct RevisionStorageClient {
    client: Arc<Client>,
    bucket: String,
}

impl RevisionStorageClient {
    pub fn new(client: Client, bucket: String) -> Self {
        Self {
            client: Arc::new(client),
            bucket,
        }
    }

    /// Download revision content (zstd decompression)
    pub async fn download_content(
        &self,
        key: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        let bytes = data.into_bytes();

        let decompressed = zstd::decode_all(bytes.as_ref())?;
        let content = String::from_utf8(decompressed)?;

        Ok(content)
    }
}
