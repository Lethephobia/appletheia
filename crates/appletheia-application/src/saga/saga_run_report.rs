use super::EnqueuedCommandCount;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    Dispatched {
        enqueued_command_count: EnqueuedCommandCount,
    },
    PredecessorRunMissing,
    AlreadyRun,
    EventAlreadyProcessed,
}
