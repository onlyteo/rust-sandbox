use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to build HTTP client: {cause}")]
    Build { cause: reqwest::Error },
    #[error("Request for user {id} failed: {cause}")]
    Request { id: Uuid, cause: reqwest::Error },
    #[error("Request for user {id} returned unexpected status {status}")]
    UnexpectedStatus { id: Uuid, status: u16 },
}

impl ClientError {
    pub fn build(cause: reqwest::Error) -> ClientError {
        ClientError::Build { cause }
    }

    pub fn request(id: Uuid, cause: reqwest::Error) -> ClientError {
        ClientError::Request { id, cause }
    }

    pub fn unexpected_status(id: Uuid, status: u16) -> ClientError {
        ClientError::UnexpectedStatus { id, status }
    }
}
