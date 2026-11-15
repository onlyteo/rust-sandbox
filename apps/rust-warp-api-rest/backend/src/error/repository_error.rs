use crate::model::OrderStatus;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Order already exists with status: {status}")]
    OrderAlreadyExists { status: OrderStatus },
    #[error("Repository lock poisoned")]
    LockPoisoned,
}

impl RepositoryError {
    pub fn order_already_exists(status: OrderStatus) -> RepositoryError {
        RepositoryError::OrderAlreadyExists { status }
    }

    pub fn lock_poisoned() -> RepositoryError {
        RepositoryError::LockPoisoned
    }
}
