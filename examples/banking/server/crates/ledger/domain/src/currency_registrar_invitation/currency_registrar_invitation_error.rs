use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    CurrencyRegistrarInvitationAcceptRejectionReason,
    CurrencyRegistrarInvitationCancelRejectionReason,
    CurrencyRegistrarInvitationDeclineRejectionReason, CurrencyRegistrarInvitationId,
    CurrencyRegistrarInvitationIssueRejectionReason, CurrencyRegistrarInvitationStateError,
};

/// Describes why an `CurrencyRegistrarInvitation` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarInvitationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarInvitationId>),

    #[error(transparent)]
    State(#[from] CurrencyRegistrarInvitationStateError),

    #[error("currency registrar invitation is already issued")]
    AlreadyIssued,
    #[error("currency registrar invitation issue rejected: {0:?}")]
    IssueRejected(CurrencyRegistrarInvitationIssueRejectionReason),
    #[error("currency registrar invitation acceptance rejected: {0:?}")]
    AcceptRejected(CurrencyRegistrarInvitationAcceptRejectionReason),
    #[error("currency registrar invitation decline rejected: {0:?}")]
    DeclineRejected(CurrencyRegistrarInvitationDeclineRejectionReason),
    #[error("currency registrar invitation cancellation rejected: {0:?}")]
    CancelRejected(CurrencyRegistrarInvitationCancelRejectionReason),
}
