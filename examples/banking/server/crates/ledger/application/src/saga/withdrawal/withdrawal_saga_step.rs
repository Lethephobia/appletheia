use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

/// Lists the logical command-dispatch steps in a withdrawal saga.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WithdrawalSagaStep {
    ReserveFunds,
    ExecuteSettlement,
    ReleaseFunds,
    CommitFunds,
    Complete,
    Fail,
}

impl SagaStep for WithdrawalSagaStep {}
