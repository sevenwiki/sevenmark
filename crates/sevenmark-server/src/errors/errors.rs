use crate::errors::handlers::{document_handler, general_handler, system_handler};
use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;
// 이 모듈은 애플리케이션의 오류 처리 시스템을 구현합니다.
// 주요 기능:
// 1. 다양한 오류 유형 정의 (사용자, 문서, 권한, 시스템 등)
// 2. 오류를 HTTP 응답으로 변환하는 메커니즘
// 3. 데이터베이스 오류를 애플리케이션 오류로 변환하는 기능

// 표준화된 Result 타입 정의
pub type ServiceResult<T> = Result<T, Errors>;
pub type ApiResult<T> = Result<T, Errors>;

// ErrorResponse 구조체: API 응답에서 오류를 표현하기 위한 구조체
// status: HTTP 상태 코드
// code: 오류 코드 문자열
// details: 개발 환경에서만 표시되는 상세 오류 메시지 (선택적)
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl From<DbErr> for Errors {
    fn from(err: sea_orm::DbErr) -> Self {
        Errors::DatabaseError(err.to_string())
    }
}

#[derive(Debug)]
pub enum Errors {
    // Document
    DocumentNotFound,
    DocumentRevisionNotFound,

    // 일반 오류
    BadRequestError(String), // 잘못된 요청 (추가 정보 포함)
    ValidationError(String), // 유효성 검사 오류 (추가 정보 포함)

    // 시스템 오류
    SysInternalError(String),
    DatabaseError(String), // 데이터베이스 오류 (추가 정보 포함)
    NotFound(String),      // 리소스를 찾을 수 없음 (추가 정보 포함)
}

// IntoResponse 트레이트 구현: Errors를 HTTP 응답으로 변환
// 각 오류 유형에 적절한 HTTP 상태 코드와 오류 코드를 매핑
// 중앙집중식 로깅도 여기서 처리
impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        // 도메인별 handler를 통한 중앙집중식 로깅
        document_handler::log_error(&self);
        general_handler::log_error(&self);
        system_handler::log_error(&self);

        // 도메인별 handler를 통한 HTTP 응답 매핑
        let (status, code, details) = document_handler::map_response(&self)
            .or_else(|| general_handler::map_response(&self))
            .or_else(|| system_handler::map_response(&self))
            .unwrap_or_else(|| {
                // Fallback: 처리되지 않은 에러
                error!("Unhandled error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "UNKNOWN_ERROR", None)
            });

        let body = ErrorResponse {
            status: status.as_u16(),
            code: code.to_string(),
            details,
        };

        // HTTP 응답으로 변환하여 반환
        (status, Json(body)).into_response()
    }
}

// 404 오류 처리 핸들러 함수
// 요청된 경로가 존재하지 않을 때 호출되는 전역 핸들러
pub async fn handler_404<B>(req: Request<B>) -> impl IntoResponse {
    // 요청 경로와 HTTP 메서드 추출
    let path = req.uri().path();
    let method = req.method().to_string();

    // NotFound 오류 반환 - 로깅은 IntoResponse에서 중앙집중화하여 처리
    Errors::NotFound(format!("Path {} with method {} not found", path, method))
}
