use std::pin::Pin;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    pub instance: String,
    pub timestamp: DateTime<Utc>,
}

impl ProblemDetails {
    pub fn new(problem_type: &str, title: &str, status: u16, detail: &str) -> Self {
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
        Self::new("urn:http:bad-request", "Bad Request", 400, detail)
    }

    pub fn unauthorized(detail: &str) -> Self {
        Self::new("urn:security:unauthorized", "Unauthorized", 401, detail)
    }

    pub fn forbidden(detail: &str) -> Self {
        Self::new("urn:security:forbidden", "Forbidden", 403, detail)
    }

    pub fn not_found(detail: &str) -> Self {
        Self::new("urn:http:not-found", "Not Found", 404, detail)
    }

    pub fn method_not_allowed(detail: &str) -> Self {
        Self::new(
            "urn:http:method-not-allowed",
            "Method Not Allowed",
            405,
            detail,
        )
    }

    pub fn unsupported_media_type(detail: &str) -> Self {
        Self::new(
            "urn:http:unsupported-media-type",
            "Unsupported Media Type",
            415,
            detail,
        )
    }

    pub fn internal_server_error(detail: &str) -> Self {
        Self::new(
            "urn:core:unhandled-error",
            "Internal Server Error",
            500,
            detail,
        )
    }
}

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
