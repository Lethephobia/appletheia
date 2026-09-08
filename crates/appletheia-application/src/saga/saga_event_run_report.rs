use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaEventRunReport {
    Processed {
        enqueued_command_count: EnqueuedCommandCount,
    },
    NoMatchingRoute,
    NotSubscribed,
    InstanceNotFound,
    CommandNotOwned,
    AlreadyProcessed,
    /// An external start event targets a saga name and correlation ID that already exist.
    AlreadyStarted,
}
