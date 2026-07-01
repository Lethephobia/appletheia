use serde::{Deserialize, Serialize};

/// Describes progress for the withdrawal orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawalSagaStatus {
    #[default]
    Initial,
    Requested,
    FundsReserveRequested,
    FundsReserved,
    TokenTransferRequested,
    TokenTransferred,
    ReservedFundsReleaseRequested,
    ReservedFundsReleased,
    ReservedFundsReleaseRejected,
    ReservedFundsCommitRequested,
    ReservedFundsCommitted,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
