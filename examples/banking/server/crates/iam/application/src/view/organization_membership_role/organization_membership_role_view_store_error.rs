use thiserror::Error;

/// Represents errors returned by organization membership role view stores.
#[derive(Debug, Error)]
pub enum OrganizationMembershipRoleViewStoreError {
    #[error("organization membership role view store persistence failed")]
    Persistence(#[source] Box<dyn std::error::Error + Send + Sync>),
}
