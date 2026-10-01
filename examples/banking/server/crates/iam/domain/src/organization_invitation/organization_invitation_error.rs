use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{OrganizationInvitationId, OrganizationInvitationStateError};

/// Describes why an `OrganizationInvitation` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationInvitationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationInvitationId>),

    #[error(transparent)]
    State(#[from] OrganizationInvitationStateError),

    #[error("organization invitation is already issued")]
    AlreadyIssued,

    #[error("organization invitation has expired")]
    Expired,

    #[error("organization invitation is not pending")]
    NotPending,

    #[error("organization has been removed")]
    OrganizationRemoved,

    #[error("invitee is already a member")]
    InviteeAlreadyMember,
}
