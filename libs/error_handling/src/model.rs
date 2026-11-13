use std::pin::Pin;

use crate::serde_status_code;
use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    #[serde(with = "serde_status_code")]
    pub status: StatusCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
}

impl ProblemDetails {
    pub fn new(problem_type: &str, title: &str, status: StatusCode, detail: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            error_type: problem_type.to_string(),
            title: title.to_string(),
            status,
            detail: Some(detail.to_string()),
            instance: String::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn bad_request(detail: &str) -> Self {
        Self::new(
            "urn:http:bad-request",
            "Bad Request",
            StatusCode::BAD_REQUEST,
            detail,
        )
    }

    pub fn unauthorized(detail: &str) -> Self {
        Self::new(
            "urn:security:unauthorized",
            "Unauthorized",
            StatusCode::UNAUTHORIZED,
            detail,
        )
    }

    pub fn forbidden(detail: &str) -> Self {
        Self::new(
            "urn:security:forbidden",
            "Forbidden",
            StatusCode::FORBIDDEN,
            detail,
        )
    }

    pub fn not_found(detail: &str) -> Self {
        Self::new(
            "urn:http:not-found",
            "Not Found",
            StatusCode::NOT_FOUND,
            detail,
        )
    }

    pub fn internal_server_error(detail: &str) -> Self {
        Self::new(
            "urn:core:unhandled-error",
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
            detail,
        )
    }
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let status = self.status;
        let mut response = (status, Json(self.clone())).into_response();
        response.extensions_mut().insert(self);
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
