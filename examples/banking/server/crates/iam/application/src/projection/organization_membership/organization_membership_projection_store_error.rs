use thiserror::Error;

/// Represents errors returned by membership projection stores.
#[derive(Debug, Error)]
pub enum OrganizationMembershipProjectionStoreError {
    #[error("membership projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
