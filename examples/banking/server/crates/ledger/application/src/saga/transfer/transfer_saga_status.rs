use serde::{Deserialize, Serialize};

/// Describes progress for the transfer orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    FundsReserveRequested,
    DepositRequested,
    ReservedFundsReleaseRequested,
    ReservedFundsCommitRequested,
    DepositedFundsWithdrawRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
