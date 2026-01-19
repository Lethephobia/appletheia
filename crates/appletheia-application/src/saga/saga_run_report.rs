#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunStatus {
    Applied,
    AlreadyProcessed,
    SkippedTerminal,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SagaRunReport {
    pub status: SagaRunStatus,
    pub commands_enqueued: usize,
    pub completed: bool,
    pub failed: bool,
}
