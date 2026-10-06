use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{WithdrawalId, WithdrawalStateError};

/// Describes why a `Withdrawal` aggregate operation failed.
#[derive(Debug, Error)]
pub enum WithdrawalError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<WithdrawalId>),

    #[error(transparent)]
    State(#[from] WithdrawalStateError),

    #[error("withdrawal has already been requested")]
    AlreadyRequested,

    #[error("token binding is unavailable for settlement")]
    TokenBindingUnavailable,

    #[error("withdrawal amount must be greater than zero")]
    ZeroAmount,

    #[error("withdrawal settlement has already been executed")]
    SettlementAlreadyExecuted,

    #[error("withdrawal is already succeeded")]
    AlreadySucceeded,

    #[error("withdrawal is already failed")]
    AlreadyFailed,

    #[error("withdrawal is already rejected")]
    AlreadyRejected,

    #[error("withdrawal settlement has not been executed")]
    SettlementNotExecuted,
}
