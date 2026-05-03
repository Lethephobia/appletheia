use thiserror::Error;

/// Represents errors returned by organization projection stores.
#[derive(Debug, Error)]
pub enum OrganizationProjectionStoreError {
    #[error("organization projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
