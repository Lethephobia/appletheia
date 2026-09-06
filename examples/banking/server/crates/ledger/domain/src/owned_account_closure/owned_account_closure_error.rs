use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    OwnedAccountClosureCompleteRejectionReason, OwnedAccountClosureFailRejectionReason,
    OwnedAccountClosureId, OwnedAccountClosurePageLoadRejectionReason,
    OwnedAccountClosureRecordRejectionReason, OwnedAccountClosureStateError,
};

/// Describes why an `OwnedAccountClosure` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OwnedAccountClosureError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OwnedAccountClosureId>),

    #[error(transparent)]
    State(#[from] OwnedAccountClosureStateError),

    #[error("owned account closure was already requested")]
    AlreadyRequested,
    #[error("owned account page load rejected: {0:?}")]
    PageLoadRejected(OwnedAccountClosurePageLoadRejectionReason),
    #[error("owned account close result record rejected: {0:?}")]
    RecordRejected(OwnedAccountClosureRecordRejectionReason),
    #[error("owned account closure completion rejected: {0:?}")]
    CompleteRejected(OwnedAccountClosureCompleteRejectionReason),
    #[error("owned account closure failure rejected: {0:?}")]
    FailRejected(OwnedAccountClosureFailRejectionReason),
}
