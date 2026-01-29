#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SagaRunReport {
    InProgress { enqueued_command_count: u32 },
    Succeeded,
    Failed,
    AlreadyProcessed,
    SkippedSucceeded,
    SkippedFailed,
}
