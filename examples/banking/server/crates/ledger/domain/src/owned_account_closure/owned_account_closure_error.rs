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
}
