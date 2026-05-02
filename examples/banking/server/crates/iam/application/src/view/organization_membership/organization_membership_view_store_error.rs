use thiserror::Error;

/// Represents errors returned by membership view stores.
#[derive(Debug, Error)]
pub enum OrganizationMembershipViewStoreError {
    #[error("membership view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
