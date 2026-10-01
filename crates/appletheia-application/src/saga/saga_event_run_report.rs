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

    /// An instance already exists for this saga name and start event ID.
    AlreadyStarted,
}
