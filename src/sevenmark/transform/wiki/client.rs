use super::types::{
    DocumentListResponse, DocumentNamespace, DocumentResponse, GetDocumentRequest,
    GetDocumentsBatchRequest,
};
use anyhow::{Context, Result};
use reqwest::Client as HttpClient;
use tracing::debug;

/// API endpoint constants
const ENDPOINT_GET_BATCH: &str = "/v0/documents/get_raw_batch_by_namespace_and_title";
const BATCH_SIZE_LIMIT: usize = 100;

/// Wiki 백엔드 클라이언트
#[derive(Clone)]
pub struct WikiClient {
    client: HttpClient,
    base_url: String,
}

impl WikiClient {
    /// 새 WikiClient 생성
    ///
    /// # Arguments
    /// * `client` - 재사용할 HTTP 클라이언트 (AppState에서 clone)
    /// * `base_url` - Wiki 서버 base URL (예: "http://127.0.0.1:8000")
    pub fn new(client: HttpClient, base_url: String) -> Self {
        Self { client, base_url }
    }

    /// Batch로 여러 문서 가져오기 (100개 초과시 자동으로 chunk 분할)
    ///
    /// # Arguments
    /// * `requests` - (namespace, title) 튜플 벡터
    ///
    /// # Returns
    /// * `Ok(Vec<DocumentResponse>)` - 성공적으로 가져온 문서들
    /// * `Err(...)` - 네트워크 오류 등
    pub async fn fetch_documents_batch(
        &self,
        requests: Vec<(DocumentNamespace, String)>,
    ) -> Result<Vec<DocumentResponse>> {
        if requests.is_empty() {
            debug!("No documents to fetch");
            return Ok(Vec::new());
        }

        let url = format!("{}{}", self.base_url, ENDPOINT_GET_BATCH);
        let mut all_documents = Vec::new();

        debug!("Fetching {} total documents from wiki", requests.len());

        // 100개씩 chunk로 나눠서 처리 (100개 이하면 1번만 루프)
        for (chunk_idx, chunk) in requests.chunks(BATCH_SIZE_LIMIT).enumerate() {
            let request_body = GetDocumentsBatchRequest {
                documents: chunk
                    .iter()
                    .map(|(namespace, title)| GetDocumentRequest {
                        namespace: namespace.clone(),
                        title: title.clone(),
                    })
                    .collect(),
            };

            debug!(
                "Sending batch request {} with {} documents to {}",
                chunk_idx + 1,
                chunk.len(),
                url
            );

            let response = self
                .client
                .post(&url)
                .json(&request_body)
                .send()
                .await
                .context("Failed to send batch request to wiki backend")?;

            let status = response.status();
            debug!("Received response with status: {}", status);

            if !status.is_success() {
                return Err(anyhow::anyhow!("Wiki backend returned error: {}", status));
            }

            let doc_list = response
                .json::<DocumentListResponse>()
                .await
                .context("Failed to parse batch document response")?;

            debug!(
                "Batch {} returned {} documents",
                chunk_idx + 1,
                doc_list.documents.len()
            );

            all_documents.extend(doc_list.documents);
        }

        debug!(
            "Successfully fetched {} documents total",
            all_documents.len()
        );

        Ok(all_documents)
    }
}
