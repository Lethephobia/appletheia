use thiserror::Error;

/// Represents errors returned by organization invitation view stores.
#[derive(Debug, Error)]
pub enum OrganizationInvitationViewStoreError {
    #[error("organization invitation view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
