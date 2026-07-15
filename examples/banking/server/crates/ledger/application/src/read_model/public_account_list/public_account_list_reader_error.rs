use thiserror::Error;

/// Error returned by public account list readers.
#[derive(Debug, Error)]
pub enum PublicAccountListReaderError {
    #[error("public account list reader persistence error")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
