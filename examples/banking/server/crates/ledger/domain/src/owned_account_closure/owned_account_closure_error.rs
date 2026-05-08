use appletheia::domain::AggregateError;
use thiserror::Error;

use super::OwnedAccountClosureId;

/// Describes why an `OwnedAccountClosure` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OwnedAccountClosureId>),

    #[error("owned account closure was already requested")]
    AlreadyRequested,
}
