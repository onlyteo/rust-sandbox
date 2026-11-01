use tracing_subscriber::filter::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum ObservabilityError {
    #[error("Failed to parse directive: {0}")]
    DirectiveParse(#[from] ParseError),
}
