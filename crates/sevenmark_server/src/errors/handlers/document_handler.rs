use crate::errors::errors::Errors;
use crate::errors::protocol::document::*;
use axum::http::StatusCode;
use tracing::warn;

/// 문서 관련 에러 로깅 처리
pub fn log_error(error: &Errors) {
    match error {
        // 리소스 찾을 수 없음 - warn! 레벨
        Errors::DocumentNotFound | Errors::DocumentRevisionNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        _ => {}
    }
}

/// Returns: (StatusCode, error_code, details)
pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::DocumentNotFound => Some((StatusCode::NOT_FOUND, DOCUMENT_NOT_FOUND, None)),
        Errors::DocumentRevisionNotFound => {
            Some((StatusCode::NOT_FOUND, DOCUMENT_REVISION_NOT_FOUND, None))
        }
        _ => None, // 다른 도메인의 에러는 None 반환
    }
}
