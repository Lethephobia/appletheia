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

    #[error("deposit token transfer has not been recorded")]
    TokenTransferNotRecorded,
}
