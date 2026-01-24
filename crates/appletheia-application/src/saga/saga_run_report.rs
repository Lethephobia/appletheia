#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    InProgress { commands_enqueued: usize },
    Succeeded,
    Failed,
    AlreadyProcessed,
    SkippedSucceeded,
    SkippedFailed,
}
