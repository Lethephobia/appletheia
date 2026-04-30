use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    InProgress {
        enqueued_command_count: EnqueuedCommandCount,
    },
    Succeeded,
    Failed,
    AlreadyProcessed,
    SkippedSucceeded,
    SkippedFailed,
}
