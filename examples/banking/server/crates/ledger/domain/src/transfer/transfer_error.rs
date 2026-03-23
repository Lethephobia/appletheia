use appletheia::domain::AggregateError;
use thiserror::Error;

use super::TransferId;

/// Describes why a `Transfer` aggregate operation failed.
#[derive(Debug, Error)]
pub enum TransferError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TransferId>),

    #[error("transfer is already initiated")]
    AlreadyInitiated,

    #[error("transfer source and destination accounts must differ")]
    SameAccount,

    #[error("transfer amount must be greater than zero")]
    ZeroAmount,

    #[error("transfer is already completed")]
    AlreadyCompleted,

    #[error("transfer is already failed")]
    AlreadyFailed,

    #[error("transfer is already cancelled")]
    AlreadyCancelled,
}
