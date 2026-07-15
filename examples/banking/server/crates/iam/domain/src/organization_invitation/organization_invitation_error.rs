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
}
