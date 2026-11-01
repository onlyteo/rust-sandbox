use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::error::response::ErrorBody;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Multiple hits for entity {name} with id {id}")]
    MultipleHits { name: String, id: Uuid },
    #[error("Precondition failed for entity {name} with id {id}")]
    PreconditionFailed { name: String, id: Uuid },
    #[error("Select failed for entity {name}: {cause}")]
    SelectFailed { name: String, cause: sqlx::Error },
    #[error("Insert failed for entity {name}: {cause}")]
    InsertFailed { name: String, cause: sqlx::Error },
    #[error("Update failed for entity {name}: {cause}")]
    UpdateFailed { name: String, cause: sqlx::Error },
    #[error("Update failed for entity {name}: {cause}")]
    DeleteFailed { name: String, cause: sqlx::Error },
    #[error("Transaction failed: {cause}")]
    TransactionFailed { cause: sqlx::Error },
}

impl DatabaseError {
    pub fn multiple_order_hits(id: Uuid) -> DatabaseError {
        DatabaseError::MultipleHits {
            name: "orders".to_string(),
            id,
        }
    }

    pub fn order_precondition_failed(id: Uuid) -> DatabaseError {
        DatabaseError::PreconditionFailed {
            name: "orders".to_string(),
            id,
        }
    }

    pub fn select_orders_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::SelectFailed {
            name: "orders".to_string(),
            cause,
        }
    }

    pub fn select_products_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::SelectFailed {
            name: "products".to_string(),
            cause,
        }
    }

    pub fn insert_order_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::InsertFailed {
            name: "orders".to_string(),
            cause,
        }
    }

    pub fn update_order_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::UpdateFailed {
            name: "orders".to_string(),
            cause,
        }
    }

    pub fn insert_order_items_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::InsertFailed {
            name: "order_items".to_string(),
            cause,
        }
    }

    pub fn update_order_items_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::UpdateFailed {
            name: "order_items".to_string(),
            cause,
        }
    }

    pub fn delete_order_items_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::DeleteFailed {
            name: "order_items".to_string(),
            cause,
        }
    }

    pub fn transaction_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::SelectFailed {
            name: "products".to_string(),
            cause,
        }
    }
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let status = match &self {
            DatabaseError::MultipleHits {
                name: _name,
                id: _id,
            } => StatusCode::CONFLICT,
            DatabaseError::PreconditionFailed {
                name: _name,
                id: _id,
            } => StatusCode::CONFLICT,
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
