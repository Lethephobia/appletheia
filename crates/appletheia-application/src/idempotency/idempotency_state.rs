use crate::command::CommandFailureReport;
use crate::idempotency::IdempotencyOutput;

#[derive(Clone, Debug, PartialEq)]
pub enum IdempotencyState {
    Succeeded { output: IdempotencyOutput },
    Failed { error: CommandFailureReport },
}
