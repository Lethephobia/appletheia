use thiserror::Error;

/// Error returned by public account list writers.
#[derive(Debug, Error)]
pub enum PublicAccountListItemWriterError {
    #[error("public account list item writer persistence error")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
