use thiserror::Error;

/// Represents errors returned by account view stores.
#[derive(Debug, Error)]
pub enum AccountViewStoreError {
    #[error("account view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
