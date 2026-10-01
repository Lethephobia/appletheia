use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{OwnedAccountClosureCountError, OwnedAccountClosureId, OwnedAccountClosureStateError};

#[derive(Debug, Error)]
pub enum OwnedAccountClosureError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OwnedAccountClosureId>),

    #[error(transparent)]
    State(#[from] OwnedAccountClosureStateError),

    #[error("owned account closure was already started")]
    AlreadyStarted,

    #[error("owned account closure is already completed")]
    AlreadyCompleted,

    #[error("account scan is already complete")]
    ScanAlreadyCompleted,

    #[error("page cursor does not match the next expected cursor")]
    UnexpectedCursor,

    #[error("owned account closure has unfinished scanning or pending results")]
    NotReadyToComplete,

    #[error("all requested account closure results have already been recorded")]
    NoPendingResults,

    #[error(transparent)]
    Count(#[from] OwnedAccountClosureCountError),
}
