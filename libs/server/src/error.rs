#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Failed to parse address: {0}")]
    Address(std::net::AddrParseError),
}
