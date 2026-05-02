use thiserror::Error;

/// Represents errors returned by user identity view stores.
#[derive(Debug, Error)]
pub enum UserIdentityViewStoreError {
    #[error("user identity view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
