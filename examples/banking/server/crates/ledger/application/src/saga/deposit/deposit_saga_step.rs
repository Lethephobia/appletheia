use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

/// Lists the logical command-dispatch steps in a deposit saga.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepositSagaStep {
    Deposit,
    Complete,
    Fail,
}

impl SagaStep for DepositSagaStep {}
