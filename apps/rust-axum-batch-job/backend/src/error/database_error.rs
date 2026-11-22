#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Claim users failed: {cause}")]
    ClaimUsersFailed { cause: sqlx::Error },
    #[error("Complete user sync failed: {cause}")]
    CompleteUserFailed { cause: sqlx::Error },
    #[error("Fail user sync failed: {cause}")]
    FailUserFailed { cause: sqlx::Error },
}

impl DatabaseError {
    pub fn claim_users_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::ClaimUsersFailed { cause }
    }

    pub fn complete_user_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::CompleteUserFailed { cause }
    }

    pub fn fail_user_failed(cause: sqlx::Error) -> DatabaseError {
        DatabaseError::FailUserFailed { cause }
    }
}
