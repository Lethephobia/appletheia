use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

/// Lists the logical command-dispatch steps in a transfer saga.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferSagaStep {
    ReserveFunds,
    Deposit,
    ReleaseFunds,
    CommitFunds,
    CompensateDeposit,
    Complete,
    Fail,
}

impl SagaStep for TransferSagaStep {}
