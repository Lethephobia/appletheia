use crate::command::CommandFailureReport;

#[derive(Clone, Debug, PartialEq)]
pub enum IdempotencyState {
    Succeeded { output: serde_json::Value },
    Failed { error: CommandFailureReport },
}
