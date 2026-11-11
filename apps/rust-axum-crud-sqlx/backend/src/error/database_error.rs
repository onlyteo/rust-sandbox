use uuid::Uuid;

use crate::model::OrderStatus;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Multiple orders found for id {id}")]
    MultipleOrdersFound { id: Uuid },
    #[error("Select orders failed: {cause}")]
    SelectOrdersFailed { cause: sqlx::Error },
    #[error("Select products failed: {cause}")]
    SelectProductsFailed { cause: sqlx::Error },
    #[error("Insert order failed: {cause}")]
    InsertOrderFailed { cause: sqlx::Error },
    #[error("Insert order items failed: {cause}")]
    InsertOrderItemsFailed { cause: sqlx::Error },
    #[error("Delete order items failed: {cause}")]
    DeleteOrderItemsFailed { cause: sqlx::Error },
    #[error("Transaction failed: {cause}")]
    TransactionFailed { cause: sqlx::Error },
    #[error("Order already exists with status: {status}")]
    OrderAlreadyExists { status: OrderStatus },
}

impl DatabaseError {
    pub fn multiple_orders_found(id: Uuid) -> DatabaseError {
        DatabaseError::MultipleOrdersFound { id }
    }

    pub fn select_orders_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::SelectOrdersFailed { cause }
    }

    pub fn select_products_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::SelectProductsFailed { cause }
    }

    pub fn insert_order_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::InsertOrderFailed { cause }
    }

    pub fn insert_order_items_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::InsertOrderItemsFailed { cause }
    }

    pub fn delete_order_items_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::DeleteOrderItemsFailed { cause }
    }

    pub fn transaction_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::TransactionFailed { cause }
    }

    pub fn order_already_exists(status: OrderStatus) -> DatabaseError {
        DatabaseError::OrderAlreadyExists { status }
    }
}
