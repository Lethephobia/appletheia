use serde::{Deserialize, Serialize};

/// Describes progress for the withdrawal orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawalSagaStatus {
    #[default]
    Initial,
    FundsReserveRequested,
    TokenTransferRequested,
    ReservedFundsReleaseRequested,
    ReservedFundsCommitRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
