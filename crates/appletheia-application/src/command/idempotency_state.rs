use super::{CommandFailureReport, IdempotencyOutput};

#[derive(Clone, Debug, PartialEq)]
pub enum IdempotencyState {
    Succeeded { output: IdempotencyOutput },
    Failed { error: CommandFailureReport },
}
