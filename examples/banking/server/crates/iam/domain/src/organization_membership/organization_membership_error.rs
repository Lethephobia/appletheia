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

    #[error("organization membership has been removed")]
    Removed,

    #[error("organization has been removed")]
    OrganizationRemoved,

    #[error("user has been removed")]
    UserRemoved,

    #[error("user is inactive")]
    UserInactive,

    #[error("user is already a member")]
    AlreadyMember,
}
