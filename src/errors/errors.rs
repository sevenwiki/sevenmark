use crate::config::db_config::DbConfig;
use crate::errors::protocol::general::{BAD_REQUEST, VALIDATION_ERROR};
use crate::errors::protocol::system::{
    SYS_DATABASE_ERROR, SYS_INTERNAL_ERROR, SYS_NOT_FOUND,

};
use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use tracing::{debug, error, warn};
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
    // 일반 오류
    BadRequestError(String),   // 잘못된 요청 (추가 정보 포함)
    ValidationError(String),   // 유효성 검사 오류 (추가 정보 포함)

    // 시스템 오류
    SysInternalError(String),
    DatabaseError(String),      // 데이터베이스 오류 (추가 정보 포함)
    NotFound(String),           // 리소스를 찾을 수 없음 (추가 정보 포함)
}

// IntoResponse 트레이트 구현: Errors를 HTTP 응답으로 변환
// 각 오류 유형에 적절한 HTTP 상태 코드와 오류 코드를 매핑
// 중앙집중식 로깅도 여기서 처리
impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        // 에러 레벨에 따른 중앙집중식 로깅
        match &self {
            // 시스템 심각도 에러 - error! 레벨
            Errors::SysInternalError(_)
            | Errors::DatabaseError(_) => {
                error!("System error occurred: {:?}", self);
            }

            // 비즈니스 로직 에러 - debug! 레벨 (클라이언트 실수)
            Errors::BadRequestError(_)
            | Errors::ValidationError(_)
            | Errors::NotFound(_) => {
                debug!("Client error: {:?}", self);
            }
        }

        // 오류 유형에 따라 상태 코드, 오류 코드, 상세 정보를 결정
        let (status, code, details) = match self {
            // 사용자 관련 오류 - 주로 401 Unauthorized 또는 404 Not Found

            // 일반 오류 - 400 Bad Request
            Errors::BadRequestError(msg) => (StatusCode::BAD_REQUEST, BAD_REQUEST, Some(msg)),
            Errors::ValidationError(msg) => (StatusCode::BAD_REQUEST, VALIDATION_ERROR, Some(msg)),


            // 시스템 오류 - 주로 500 Internal Server Error
            Errors::SysInternalError(msg) => {
                (StatusCode::BAD_REQUEST, SYS_INTERNAL_ERROR, Some(msg))
            }

            Errors::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SYS_DATABASE_ERROR,
                Some(msg),
            ),
            Errors::NotFound(msg) => (StatusCode::NOT_FOUND, SYS_NOT_FOUND, Some(msg)),

        };

        // 개발 환경에서만 상세 오류 정보 포함
        let is_dev = DbConfig::get().is_dev;

        // 오류 응답 구성
        let body = ErrorResponse {
            status: status.as_u16(),
            code: code.to_string(),
            details: if is_dev { details } else { None }, // 개발 환경에서만 상세 정보 표시
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