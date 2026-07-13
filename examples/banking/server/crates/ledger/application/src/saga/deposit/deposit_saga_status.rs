use serde::{Deserialize, Serialize};

/// Describes progress for the deposit orchestration saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepositSagaStatus {
    #[default]
    Initial,
    AccountDepositRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
