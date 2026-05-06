use appletheia::domain::AggregateError;
use thiserror::Error;

use super::TransferId;

/// Describes why a `Transfer` aggregate operation failed.
#[derive(Debug, Error)]
pub enum TransferError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TransferId>),

    #[error("transfer has already been requested")]
    AlreadyRequested,
}
