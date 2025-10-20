use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 문서 namespace (백엔드 API 스펙 & DB enum)
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ToSchema, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "document_namespace")]
pub enum DocumentNamespace {
    #[sea_orm(string_value = "document")]
    Document,
    #[sea_orm(string_value = "file")]
    File,
    #[sea_orm(string_value = "category")]
    Category,
}

/// 문서 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResponse {
    pub id: String,
    pub namespace: DocumentNamespace,
    pub title: String,
    pub current_revision: DocumentRevision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}

/// 문서 revision 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub content: String,
}
