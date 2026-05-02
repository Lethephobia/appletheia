use thiserror::Error;

/// Represents errors returned by user view stores.
#[derive(Debug, Error)]
pub enum UserViewStoreError {
    #[error("user view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
