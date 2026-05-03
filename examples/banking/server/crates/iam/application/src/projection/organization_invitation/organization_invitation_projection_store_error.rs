use thiserror::Error;

/// Represents errors returned by organization invitation projection stores.
#[derive(Debug, Error)]
pub enum OrganizationInvitationProjectionStoreError {
    #[error("organization invitation projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
