use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    DepositCompleteRejectionReason, DepositFailRejectionReason, DepositId,
    DepositRequestRejectionReason, DepositSettlementVerifyRejectionReason, DepositStateError,
};

/// Describes why a `Deposit` aggregate operation failed.
#[derive(Debug, Error)]
pub enum DepositError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<DepositId>),

    #[error(transparent)]
    State(#[from] DepositStateError),

    #[error("deposit has already been requested")]
    AlreadyRequested,

    #[error("deposit settlement has not been verified")]
    SettlementNotVerified,
    #[error("deposit request rejected: {0:?}")]
    RequestRejected(DepositRequestRejectionReason),
    #[error("deposit settlement verification rejected: {0:?}")]
    SettlementVerifyRejected(DepositSettlementVerifyRejectionReason),
    #[error("deposit completion rejected: {0:?}")]
    CompleteRejected(DepositCompleteRejectionReason),
    #[error("deposit failure rejected: {0:?}")]
    FailRejected(DepositFailRejectionReason),
}
