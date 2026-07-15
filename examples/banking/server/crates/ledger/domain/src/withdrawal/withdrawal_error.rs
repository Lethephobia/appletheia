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
}
