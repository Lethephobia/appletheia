use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    WithdrawalCompleteRejectionReason, WithdrawalFailRejectionReason, WithdrawalId,
    WithdrawalRequestRejectionReason, WithdrawalSettlementExecuteRejectionReason,
    WithdrawalStateError,
};

/// Describes why a `Withdrawal` aggregate operation failed.
#[derive(Debug, Error)]
pub enum WithdrawalError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<WithdrawalId>),

    #[error(transparent)]
    State(#[from] WithdrawalStateError),

    #[error("withdrawal has already been requested")]
    AlreadyRequested,
    #[error("withdrawal request rejected: {0:?}")]
    RequestRejected(WithdrawalRequestRejectionReason),
    #[error("withdrawal settlement execution rejected: {0:?}")]
    SettlementExecuteRejected(WithdrawalSettlementExecuteRejectionReason),
    #[error("withdrawal completion rejected: {0:?}")]
    CompleteRejected(WithdrawalCompleteRejectionReason),
    #[error("withdrawal failure rejected: {0:?}")]
    FailRejected(WithdrawalFailRejectionReason),
}
