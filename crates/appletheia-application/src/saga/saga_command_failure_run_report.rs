use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaCommandFailureRunReport {
    InProgress {
        enqueued_command_count: EnqueuedCommandCount,
    },
    Succeeded,
    Failed,
    NotSubscribed,
    InstanceNotFound,
    CommandNotOwned,
    AlreadyProcessed,
    SkippedSucceeded,
    SkippedFailed,
}
