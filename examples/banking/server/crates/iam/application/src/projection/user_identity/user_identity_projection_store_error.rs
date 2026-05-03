use thiserror::Error;

/// Represents errors returned by user identity projection stores.
#[derive(Debug, Error)]
pub enum UserIdentityProjectionStoreError {
    #[error("user identity projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
