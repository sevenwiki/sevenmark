use crate::sevenmark::transform::preprocessor::IncludeInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Include된 문서 데이터
#[derive(Debug, Clone)]
pub struct IncludeData {
    /// 문서의 원본 SevenMark 텍스트
    pub content: String,
    /// Include 정보 (title, namespace, parameters)
    pub info: IncludeInfo,
}

// ===== Backend API Types =====

/// 문서 namespace (백엔드 API 스펙)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentNamespace {
    Document,
    User,
    Template,
    File,
    Category,
    Wiki,
}

/// GET 문서 요청 (POST /v0/documents/get_raw_by_namespace_and_title)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    pub namespace: DocumentNamespace,
    pub title: String,
}

/// 문서 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResponse {
    pub id: String,
    pub namespace: DocumentNamespace,
    pub title: String,
    pub current_revision: DocumentRevision,
}



/// 문서 revision 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub content: String,
    pub file_url: Option<String>,
}

// ===== Batch API Types =====

/// Batch 문서 요청 (POST /v0/documents/get_raw_batch_by_namespace_and_title)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentsBatchRequest {
    pub documents: Vec<GetDocumentRequest>,
}

/// Batch 문서 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentResponse>,
}
