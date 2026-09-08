use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaCommandFailureRunReport {
    Processed {
        enqueued_command_count: EnqueuedCommandCount,
    },
    NoMatchingRoute,
    NotSubscribed,
    InstanceNotFound,
    CommandNotOwned,
    AlreadyProcessed,
}
