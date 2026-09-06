use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    CurrencyRegistrarJoinRequestApproveRejectionReason,
    CurrencyRegistrarJoinRequestCancelRejectionReason, CurrencyRegistrarJoinRequestId,
    CurrencyRegistrarJoinRequestRejectRejectionReason, CurrencyRegistrarJoinRequestStateError,
    CurrencyRegistrarJoinRequestSubmitRejectionReason,
};

/// Describes why an `CurrencyRegistrarJoinRequest` aggregate operation failed.
#[derive(Debug, Error)]
pub enum CurrencyRegistrarJoinRequestError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CurrencyRegistrarJoinRequestId>),

    #[error(transparent)]
    State(#[from] CurrencyRegistrarJoinRequestStateError),

    #[error("currency registrar join request is already submitted")]
    AlreadySubmitted,
    #[error("currency registrar join request submission rejected: {0:?}")]
    SubmitRejected(CurrencyRegistrarJoinRequestSubmitRejectionReason),
    #[error("currency registrar join request approval rejected: {0:?}")]
    ApproveRejected(CurrencyRegistrarJoinRequestApproveRejectionReason),
    #[error("currency registrar join request rejection rejected: {0:?}")]
    RejectRejected(CurrencyRegistrarJoinRequestRejectRejectionReason),
    #[error("currency registrar join request cancellation rejected: {0:?}")]
    CancelRejected(CurrencyRegistrarJoinRequestCancelRejectionReason),
}
