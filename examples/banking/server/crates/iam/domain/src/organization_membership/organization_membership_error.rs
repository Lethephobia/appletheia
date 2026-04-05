use appletheia::domain::AggregateError;
use thiserror::Error;

use super::OrganizationMembershipId;

/// Describes why an `OrganizationMembership` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationMembershipId>),

    #[error("organization membership is already created")]
    AlreadyCreated,

    #[error("organization membership is removed")]
    Removed,
}
