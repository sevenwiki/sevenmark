use super::types::{DocumentNamespace, DocumentResponse, GetDocumentRequest};
use anyhow::{Context, Result};
use reqwest::Client as HttpClient;

/// API endpoint constants
const ENDPOINT_GET_DOCUMENT: &str = "/v0/documents/get_raw_by_namespace_and_title";
const ENDPOINT_GET_BATCH: &str = "/v0/documents/get_raw_batch_by_namespace_and_title";

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

    /// 문서 가져오기
    ///
    /// # Arguments
    /// * `namespace` - 문서 네임스페이스
    /// * `title` - 문서 제목
    ///
    /// # Returns
    /// * `Ok(Some(DocumentResponse))` - 문서 존재
    /// * `Ok(None)` - 문서 없음 (404)
    /// * `Err(...)` - 네트워크 오류 등
    pub async fn fetch_document(
        &self,
        namespace: DocumentNamespace,
        title: &str,
    ) -> Result<Option<DocumentResponse>> {
        let url = format!("{}{}", self.base_url, ENDPOINT_GET_DOCUMENT);

        let request_body = GetDocumentRequest {
            namespace,
            title: title.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to wiki backend")?;

        match response.status() {
            status if status.is_success() => {
                let doc = response
                    .json::<DocumentResponse>()
                    .await
                    .context("Failed to parse document response")?;
                Ok(Some(doc))
            }
            status if status.as_u16() == 404 => Ok(None),
            status => Err(anyhow::anyhow!("Wiki backend returned error: {}", status)),
        }
    }

    /// Batch로 여러 문서 가져오기
    ///
    /// # Arguments
    /// * `requests` - (namespace, title) 튜플 벡터 (최대 100개)
    ///
    /// # Returns
    /// * `Ok(Vec<DocumentResponse>)` - 성공적으로 가져온 문서들
    /// * `Err(...)` - 네트워크 오류 등
    pub async fn fetch_documents_batch(
        &self,
        requests: Vec<(DocumentNamespace, String)>,
    ) -> Result<Vec<DocumentResponse>> {
        use super::types::{DocumentListResponse, GetDocumentsBatchRequest};

        if requests.is_empty() {
            return Ok(Vec::new());
        }

        let url = format!("{}{}", self.base_url, ENDPOINT_GET_BATCH);

        let request_body = GetDocumentsBatchRequest {
            documents: requests
                .into_iter()
                .map(|(namespace, title)| GetDocumentRequest { namespace, title })
                .collect(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send batch request to wiki backend")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Wiki backend returned error: {}",
                response.status()
            ));
        }

        let doc_list = response
            .json::<DocumentListResponse>()
            .await
            .context("Failed to parse batch document response")?;

        Ok(doc_list.documents)
    }
}
