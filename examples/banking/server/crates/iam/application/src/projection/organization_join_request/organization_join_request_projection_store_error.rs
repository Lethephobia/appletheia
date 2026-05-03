use thiserror::Error;

/// Represents errors returned by organization join request projection stores.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestProjectionStoreError {
    #[error("organization join request projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
