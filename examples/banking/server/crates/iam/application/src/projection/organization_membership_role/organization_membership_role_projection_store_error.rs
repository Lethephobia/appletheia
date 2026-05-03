use thiserror::Error;

/// Represents errors returned by organization membership role projection stores.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRoleProjectionStoreError {
    #[error("organization membership role projection store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
