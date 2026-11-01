use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Order with id {id} not found")]
    OrderNotFound { id: Uuid },
    #[error("Could not create order")]
    CreateOrderFailed,
}

impl AppError {
    pub fn order_not_found(id: Uuid) -> AppError {
        AppError::OrderNotFound { id }
    }

    pub fn create_order_failed() -> AppError {
        AppError::CreateOrderFailed
    }
}
