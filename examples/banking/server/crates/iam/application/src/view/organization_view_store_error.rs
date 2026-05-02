use thiserror::Error;

/// Represents errors returned by organization view stores.
#[derive(Debug, Error)]
pub enum OrganizationViewStoreError {
    #[error("organization view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
