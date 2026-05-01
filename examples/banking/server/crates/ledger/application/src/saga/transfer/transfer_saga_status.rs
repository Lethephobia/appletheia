use serde::{Deserialize, Serialize};

/// Describes progress for the transfer orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    Requested,
    FundsReserved,
    Deposited,
    ReservedFundsReleaseRequested,
    ReservedFundsReleaseRejected,
    ReservedFundsCommitted,
    ReservedFundsCommitRejected,
    ReservedFundsReleased,
    DepositedFundsWithdrawn,
    FailRequested,
    Completed,
    Failed,
}
