use thiserror::Error;

/// Error returned by public account list writers.
#[derive(Debug, Error)]
pub enum PublicAccountListWriterError {
    #[error("public account list writer persistence error")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
