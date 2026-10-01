use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{DepositId, DepositStateError};

/// Describes why a `Deposit` aggregate operation failed.
#[derive(Debug, Error)]
pub enum DepositError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<DepositId>),

    #[error(transparent)]
    State(#[from] DepositStateError),

    #[error("deposit has already been requested")]
    AlreadyRequested,

    #[error("deposit settlement has not been verified")]
    SettlementNotVerified,

    #[error("deposit is already completed")]
    AlreadyCompleted,

    #[error("deposit is already failed")]
    AlreadyFailed,

    #[error("deposit amount must be greater than zero")]
    ZeroAmount,

    #[error("token binding is unavailable for settlement")]
    TokenBindingUnavailable,

    #[error("transaction does not match the settlement chain")]
    ChainMismatch,

    #[error("deposit settlement has already been verified")]
    SettlementAlreadyVerified,

    #[error("deposit is already rejected")]
    AlreadyRejected,
}
