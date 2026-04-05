use appletheia::domain::AggregateError;
use thiserror::Error;

use super::OrganizationInvitationId;

/// Describes why an `OrganizationInvitation` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationInvitationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationInvitationId>),

    #[error("organization invitation is already issued")]
    AlreadyIssued,

    #[error("organization invitation is not pending")]
    NotPending,

    #[error("organization invitation is expired")]
    Expired,
}
