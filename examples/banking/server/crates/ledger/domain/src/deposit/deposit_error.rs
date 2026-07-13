use appletheia::domain::AggregateError;
use thiserror::Error;

use super::DepositId;

/// Describes why a `Deposit` aggregate operation failed.
#[derive(Debug, Error)]
pub enum DepositError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<DepositId>),

    #[error("deposit has already been requested")]
    AlreadyRequested,

    #[error("deposit token transfer has not been recorded")]
    TokenTransferNotRecorded,
}
