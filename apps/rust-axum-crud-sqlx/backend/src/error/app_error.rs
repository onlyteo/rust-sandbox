use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::error::DatabaseError;
use crate::error::response::ErrorBody;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Order with id {id} not found")]
    OrderNotFound { id: Uuid },
    #[error("Could not create order")]
    CreateOrderFailed,
    #[error("{0}")]
    DatabaseError(DatabaseError),
}

impl AppError {
    pub fn order_not_found(id: Uuid) -> AppError {
        AppError::OrderNotFound { id }
    }

    pub fn create_order_failed() -> AppError {
        AppError::CreateOrderFailed
    }
}

impl From<DatabaseError> for AppError {
    fn from(error: DatabaseError) -> Self {
        AppError::DatabaseError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::OrderNotFound { id: _id } => StatusCode::NOT_FOUND,
            AppError::CreateOrderFailed => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        tracing::error!("{self}"); // Error logging before API reponse is returned
        (
            status,
            Json(ErrorBody {
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
