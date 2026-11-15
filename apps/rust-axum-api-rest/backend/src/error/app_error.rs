use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_handling::model::ProblemDetails;
use uuid::Uuid;

use crate::error::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Order with id {id} not found")]
    OrderNotFound { id: Uuid },
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

impl AppError {
    pub fn order_not_found(id: Uuid) -> AppError {
        AppError::OrderNotFound { id }
    }
}

impl Into<ProblemDetails> for AppError {
    fn into(self) -> ProblemDetails {
        match &self {
            AppError::OrderNotFound { .. } => ProblemDetails::new(
                "urn:app:order-not-found",
                "Order Not Found",
                StatusCode::NOT_FOUND.as_u16(),
                &self.to_string(),
            ),
            AppError::Repository(cause) => match cause {
                RepositoryError::OrderAlreadyExists { .. } => ProblemDetails::new(
                    "urn:app:order-already-exists",
                    "Order Already Exists",
                    StatusCode::CONFLICT.as_u16(),
                    &self.to_string(),
                ),
                _ => ProblemDetails::internal_server_error("An internal error occurred"),
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}"); // Error logging before API response is returned
        let problem: ProblemDetails = self.into();
        problem.into_response()
    }
}
