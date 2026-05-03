use thiserror::Error;

/// Represents errors returned by user projection stores.
#[derive(Debug, Error)]
pub enum UserProjectionStoreError {
    #[error("user projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
