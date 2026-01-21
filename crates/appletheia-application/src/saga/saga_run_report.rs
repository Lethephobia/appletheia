#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    InProgress { commands_enqueued: usize },
    Succeeded { commands_enqueued: usize },
    Failed { commands_enqueued: usize },
    AlreadyProcessed,
    SkippedSucceeded,
    SkippedFailed,
}
