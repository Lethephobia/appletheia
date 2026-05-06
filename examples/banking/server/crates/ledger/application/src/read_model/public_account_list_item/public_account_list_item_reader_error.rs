use thiserror::Error;

/// Error returned by public account list readers.
#[derive(Debug, Error)]
pub enum PublicAccountListItemReaderError {
    #[error("public account list item reader persistence error")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
