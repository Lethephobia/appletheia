use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{CurrencyRegistrarInvitationId, CurrencyRegistrarInvitationStateError};

/// Describes why an `CurrencyRegistrarInvitation` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarInvitationId>),

    #[error(transparent)]
    State(#[from] CurrencyRegistrarInvitationStateError),

    #[error("currency registrar invitation is already issued")]
    AlreadyIssued,

    #[error("currency registrar invitation has expired")]
    Expired,

    #[error("currency registrar invitation is not pending")]
    NotPending,

    #[error("invitee is already a member")]
    InviteeAlreadyMember,
}
