#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    CommandDispatched,
    NoCommandDispatched,
    PredecessorRunMissing,
    AlreadyRun,
    EventAlreadyProcessed,
}
