use super::client::WikiClient;
use super::types::{DocumentNamespace, IncludeData, WikiData};
use crate::sevenmark::processor::preprocessor::PreprocessInfo;
use anyhow::Result;
use tracing::{debug, warn};

/// Wiki 데이터 해결기
pub struct WikiResolver;

impl WikiResolver {
    /// PreprocessInfo에서 수집된 정보를 바탕으로 Wiki 데이터 해결
    ///
    /// # Arguments
    /// * `info` - preprocessor에서 수집한 정보
    /// * `client` - Wiki 백엔드 클라이언트
    ///
    /// # Returns
    /// 해결된 Wiki 데이터 (includes만 채워짐)
    pub async fn resolve(info: &PreprocessInfo, client: &WikiClient) -> Result<WikiData> {
        let mut wiki_data = WikiData::default();

        // Include 문서들 fetch
        for (key, include_info) in &info.includes {
            // namespace를 parameters에서 추출 (기본값: "Document")
            let namespace_str = include_info
                .parameters
                .get("namespace")
                .cloned()
                .unwrap_or_else(|| "Document".to_string());

            debug!(
                "Fetching include document: {} (namespace: {})",
                include_info.title, namespace_str
            );

            // namespace 문자열을 DocumentNamespace enum으로 변환
            let namespace = Self::parse_namespace(&namespace_str);

            match client
                .fetch_document(namespace, &include_info.title)
                .await
            {
                Ok(Some(doc)) => {
                    wiki_data.includes.insert(
                        key.clone(),
                        IncludeData {
                            content: doc.current_revision.content,
                            info: include_info.clone(),
                        },
                    );
                }
                Ok(None) => {
                    warn!("Include document not found: {}", key);
                }
                Err(e) => {
                    warn!("Failed to fetch include '{}': {}", key, e);
                }
            }
        }

        Ok(wiki_data)
    }

    /// namespace 문자열을 DocumentNamespace enum으로 변환
    fn parse_namespace(namespace: &str) -> DocumentNamespace {
        match namespace {
            "Document" => DocumentNamespace::Document,
            "User" => DocumentNamespace::User,
            "Template" => DocumentNamespace::Template,
            "File" => DocumentNamespace::File,
            "Category" => DocumentNamespace::Category,
            "Wiki" => DocumentNamespace::Wiki,
            _ => DocumentNamespace::Document, // 기본값
        }
    }
}