use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{TransferId, TransferStateError};

/// Describes why a `Transfer` aggregate operation failed.
#[derive(Debug, Error)]
pub enum TransferError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TransferId>),

    #[error(transparent)]
    State(#[from] TransferStateError),

    #[error("transfer has already been requested")]
    AlreadyRequested,

    #[error("transfer is already completed")]
    AlreadyCompleted,

    #[error("transfer is already failed")]
    AlreadyFailed,

    #[error("transfer is already rejected")]
    AlreadyRejected,

    #[error("source and destination accounts use different currencies")]
    CurrencyMismatch,

    #[error("source and destination accounts must be different")]
    SameSourceAndDestinationAccount,

    #[error("transfer amount must be greater than zero")]
    ZeroAmount,
}
