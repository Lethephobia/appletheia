use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    TransferCompleteRejectionReason, TransferFailRejectionReason, TransferId,
    TransferRequestRejectionReason, TransferStateError,
};

/// Describes why a `Transfer` aggregate operation failed.
#[derive(Debug, Error)]
pub enum TransferError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<TransferId>),

    #[error(transparent)]
    State(#[from] TransferStateError),

    #[error("transfer has already been requested")]
    AlreadyRequested,
    #[error("transfer request rejected: {0:?}")]
    RequestRejected(TransferRequestRejectionReason),
    #[error("transfer completion rejected: {0:?}")]
    CompleteRejected(TransferCompleteRejectionReason),
    #[error("transfer failure rejected: {0:?}")]
    FailRejected(TransferFailRejectionReason),
}
