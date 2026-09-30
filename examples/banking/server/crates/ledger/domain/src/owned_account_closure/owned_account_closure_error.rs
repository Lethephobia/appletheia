use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{OwnedAccountClosureId, OwnedAccountClosureStateError};

/// Describes why an `OwnedAccountClosure` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OwnedAccountClosureId>),

    #[error(transparent)]
    State(#[from] OwnedAccountClosureStateError),

    #[error("owned account closure was already requested")]
    AlreadyRequested,

    #[error("owned account closure has already finished")]
    AlreadyFinished,

    #[error("owned account closure is not in progress")]
    NotInProgress,

    #[error("one or more accounts could not be closed")]
    AccountClosureFailed,

    #[error("owned account closure is already completed")]
    AlreadyCompleted,

    #[error("owned account closure is already failed")]
    AlreadyFailed,
}
