use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{OrganizationJoinRequestId, OrganizationJoinRequestStateError};

/// Describes why an `OrganizationJoinRequest` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationJoinRequestError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationJoinRequestId>),

    #[error(transparent)]
    State(#[from] OrganizationJoinRequestStateError),

    #[error("organization join request is already submitted")]
    AlreadySubmitted,

    #[error("organization join request is not pending")]
    NotPending,

    #[error("organization has been removed")]
    OrganizationRemoved,

    #[error("requester is already a member")]
    RequesterAlreadyMember,
}
