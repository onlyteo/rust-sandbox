use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use error_handling::model::ProblemDetails;
use uuid::Uuid;

use crate::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Order with id {id} not found")]
    OrderNotFound { id: Uuid },
    #[error("Could not create order")]
    CreateOrderFailed,
    #[error(transparent)]
    Database(#[from] DatabaseError),
}

impl AppError {
    pub fn order_not_found(id: Uuid) -> AppError {
        AppError::OrderNotFound { id }
    }

    pub fn create_order_failed() -> AppError {
        AppError::CreateOrderFailed
    }

    fn problem(&self) -> ProblemDetails {
        match self {
            AppError::OrderNotFound { .. } => ProblemDetails::new(
                "urn:app:order-not-found",
                "Order Not Found",
                StatusCode::NOT_FOUND,
                &self.to_string(),
            ),
            AppError::CreateOrderFailed => ProblemDetails::new(
                "urn:app:create-order-failed",
                "Create Order Failed",
                StatusCode::INTERNAL_SERVER_ERROR,
                &self.to_string(),
            ),
            AppError::Database(_) => {
                ProblemDetails::internal_server_error("An internal error occurred")
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}"); // Error logging before API response is returned
        self.problem().into_response()
    }
}
