use thiserror::Error;

/// Represents errors returned by organization join request view stores.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestViewStoreError {
    #[error("organization join request view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
