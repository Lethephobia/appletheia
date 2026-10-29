use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{OrganizationMembershipId, OrganizationMembershipStateError};

/// Describes why an `OrganizationMembership` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationMembershipId>),

    #[error(transparent)]
    State(#[from] OrganizationMembershipStateError),

    #[error("organization membership is already created")]
    AlreadyCreated,

    #[error("organization membership has been deleted")]
    Deleted,

    #[error("organization has been deleted")]
    OrganizationDeleted,

    #[error("user has been deleted")]
    UserDeleted,

    #[error("user is inactive")]
    UserInactive,

    #[error("user is already a member")]
    AlreadyMember,
}
