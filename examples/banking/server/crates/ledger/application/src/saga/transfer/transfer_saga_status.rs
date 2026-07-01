use serde::{Deserialize, Serialize};

/// Describes progress for the transfer orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    Requested,
    FundsReserveRequested,
    FundsReserved,
    DepositRequested,
    Deposited,
    ReservedFundsReleaseRequested,
    ReservedFundsReleaseRejected,
    ReservedFundsCommitRequested,
    ReservedFundsCommitted,
    ReservedFundsCommitRejected,
    DepositedFundsWithdrawRequested,
    ReservedFundsReleased,
    DepositedFundsWithdrawn,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
