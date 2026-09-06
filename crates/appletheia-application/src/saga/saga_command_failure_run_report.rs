use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaCommandFailureRunReport {
    InProgress {
        enqueued_command_count: EnqueuedCommandCount,
    },
    Completed,
    NotSubscribed,
    InstanceNotFound,
    CommandNotOwned,
    AlreadyProcessed,
    SkippedCompleted,
}
